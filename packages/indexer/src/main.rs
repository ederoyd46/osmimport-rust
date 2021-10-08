#![allow(dead_code)]
mod protos;
mod types;
mod utils;

use protos::fileformat::{Blob, BlockHeader};
use protos::osmformat::{DenseNodes, HeaderBlock, PrimitiveBlock, StringTable};
use types::Node;
use utils::{calculate_degrees, delta_decode, get_datetime, NANO};

use geojson::{Feature, GeoJson, Geometry, Value};
use protobuf::Message;
use serde_json::{to_value, Map};
use std::collections::HashMap;
use std::convert::TryInto;
use std::env;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::Path;
use std::str;

use flate2::{Decompress, FlushDecompress};

fn main() {
    println!("--- Start ---");
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let query_path = &args[1];
    let path = Path::new(query_path);
    start_import(path);
    println!("--- End ---");
}

fn start_import(path: &Path) {
    match path.is_file() {
        true => {
            let mut data_file = File::open(&path).expect("File does not exist");
            // Header
            get_block(&mut data_file).expect("Can't read header");
            // Data
            for _x in 0.. {
                get_block(&mut data_file).expect("No more data");
            }
        }
        false => panic!("This is not a file"),
    }
}

fn get_block(file: &mut File) -> Result<(), &str> {
    let mut header_size_buffer = [0; 4];

    file.read_exact(&mut header_size_buffer)
        .expect("Could not read file");
    let header_size = u32::from_be_bytes(header_size_buffer);

    let mut header_buffer = vec![
        0;
        header_size
            .try_into()
            .expect("Could not convert header size")
    ];
    file.read_exact(&mut header_buffer).unwrap();

    let header = BlockHeader::parse_from_bytes(&header_buffer).expect("Header not valid");

    let mut blob_buffer = vec![
        0;
        header
            .get_datasize()
            .try_into()
            .expect("Could not convert buffer size")
    ];

    file.read_exact(&mut blob_buffer).unwrap();

    let blob_data = Blob::parse_from_bytes(&blob_buffer).expect("Blob Data not valid");
    let mut uncompressed_blob_buffer = vec![
        0;
        blob_data
            .get_raw_size()
            .try_into()
            .expect("Could not convert raw size")
    ];

    Decompress::new(true)
        .decompress(
            blob_data.get_zlib_data(),
            &mut uncompressed_blob_buffer,
            FlushDecompress::Sync,
        )
        .expect("Data Could not be uncompressed");

    if header.get_field_type() == "OSMHeader" {
        let result = load_proto_message::<HeaderBlock>(uncompressed_blob_buffer);
        handle_header_block(result);
    } else {
        let result = load_proto_message::<PrimitiveBlock>(uncompressed_blob_buffer);
        handle_data_block(result);
    }

    Ok(())
}

fn load_proto_message<T: Message>(data: Vec<u8>) -> T {
    T::parse_from_bytes(&data)
        .unwrap_or_else(|_| panic!("Could not load decompressed block with data {:?}", data))
}

fn handle_header_block(block: HeaderBlock) {
    let bbox = block.get_bbox();

    println!("Left {}", bbox.get_left() as f64 / NANO);
    println!("Right {}", bbox.get_right() as f64 / NANO);
    println!("Top {}", bbox.get_top() as f64 / NANO);
    println!("Bottom {}", bbox.get_bottom() as f64 / NANO);
    println!("Base URL {}", block.get_osmosis_replication_base_url());
    println!(
        "Sequence Number {}",
        block.get_osmosis_replication_sequence_number()
    );
    println!(
        "Replication Timestamp {}",
        get_datetime(block.get_osmosis_replication_timestamp())
    );
}

fn handle_data_block(block: PrimitiveBlock) {
    let string_table = convert_string_table(block.get_stringtable());
    let granularity = block.get_granularity() as f64;
    let date_granularity = block.get_date_granularity() as i64;
    let primitive_groups = block.get_primitivegroup().iter();
    println!("String Table = {:?}", &string_table.len());
    println!("Granularity = {:?}", granularity);
    println!("Date Granularity = {:?}", date_granularity);
    println!("Primitive Groups = {:?}", primitive_groups.len());
    for group in primitive_groups {
        let nodes = handle_dense_nodes(group.get_dense(), &string_table, granularity);
        let place_nodes: Vec<&Node> = nodes
            .iter()
            .filter(|n| n.tags.contains_key("place") && n.tags.contains_key("name"))
            .collect();

        write_geo_json_files(place_nodes);
    }
}

fn handle_dense_nodes(nodes: &DenseNodes, string_table: &[&str], granularity: f64) -> Vec<Node> {
    let mut result: Vec<Node> = vec![];

    let size = nodes.get_id().len();
    let ids = delta_decode(0, nodes.get_id());
    let timestamps = delta_decode(0, nodes.get_denseinfo().get_timestamp());
    let latitudes = delta_decode(0, nodes.get_lat());
    let longitudes = delta_decode(0, nodes.get_lon());
    let tags = build_key_vals(nodes.get_keys_vals(), string_table);

    for i in 0..size {
        let node = Node {
            id: ids[i],
            latitude: calculate_degrees(latitudes[i], granularity),
            longitude: calculate_degrees(longitudes[i], granularity),
            timestamp: get_datetime(timestamps[i]),
            tags: tags[i].clone(),
        };

        if !node.tags.is_empty() {
            result.push(node);
        }
    }

    result
}

fn build_key_vals(mixed_key_vals: &[i32], string_table: &[&str]) -> Vec<HashMap<String, String>> {
    let mut results: Vec<HashMap<String, String>> = Vec::new();
    let mut itr = mixed_key_vals.to_vec().into_iter();
    let mut current: HashMap<String, String> = HashMap::new();
    while let Some(key) = itr.next() {
        if key == 0 {
            results.push(current); // 0 marks the end of the previous list
            current = HashMap::new();
        } else {
            current.insert(
                String::from(string_table[key as usize]),
                String::from(string_table[itr.next().unwrap() as usize]),
            );
        }
    }
    results
}

fn convert_string_table(string_table: &StringTable) -> Vec<&str> {
    string_table
        .get_s()
        .iter()
        .map(|x| str::from_utf8(x).unwrap())
        .collect()
}

fn write_geo_json_files(nodes: Vec<&Node>) {
    for node in nodes {
        write_geo_json_file(node);
    }
}

fn create_file_structure(node: &Node) -> File {
    let place = clean_name(node.tags.get("place").unwrap());
    let name = clean_name(node.tags.get("name").unwrap());
    let path_str = format!("index/{}", place);
    create_dir_all(&path_str).unwrap();

    let path_file_str = format!("{}/{}.json", path_str, name);
    let path = Path::new(&path_file_str);

    let file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };

    file
}

fn clean_name(name: &str) -> String {
    name.trim()
        .replace("/", "")
        .replace('\\', "")
        .replace(" ", "+")
        .to_lowercase()
}

fn write_geo_json_file(node: &Node) {
    let mut properties = Map::new();
    for i in &node.tags {
        properties.insert(String::from(i.0), to_value(i.1).unwrap());
    }

    let geojson = GeoJson::Feature(Feature {
        bbox: None,
        geometry: Some(Geometry::new(Value::Point(vec![
            node.longitude,
            node.latitude,
        ]))),
        id: None,
        properties: Some(properties),
        foreign_members: None,
    });

    let mut file = create_file_structure(node);
    file.write_all(geojson.to_string().as_bytes())
        .expect("Could not write to file");
}
