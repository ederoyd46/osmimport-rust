#![allow(dead_code)]
mod index;

mod protos;
use protos::fileformat::{Blob, BlockHeader};
use protos::osmformat::{DenseNodes, HeaderBlock, PrimitiveBlock, StringTable};

use chrono::{DateTime, NaiveDateTime, Utc};
use protobuf::{parse_from_bytes, Message};
use std::collections::HashMap;
use std::convert::TryInto;
use std::env;
use std::fmt::Display;
use std::fs::{self};
use std::io::prelude::*;
use std::ops::Add;
use std::path::Path;
use std::str;

use flate2::{Decompress, FlushDecompress};

const NANO: f64 = 1000000000.0;

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
            let mut data_file = fs::File::open(&path).expect("File does not exist");
            // Header
            get_block(&mut data_file).expect("Can't read header");
            // Data
            for x in 0..1 {
                get_block(&mut data_file).expect("No more data");
                println!("{}", x);
            }
        }
        false => panic!("This is not a file"),
    }
}

fn get_block(file: &mut fs::File) -> Result<(), &str> {
    let mut header_size_buffer = [0; 4];

    file.read(&mut header_size_buffer)
        .expect("Could not read file");
    let header_size = u32::from_be_bytes(header_size_buffer);

    let mut header_buffer = vec![
        0;
        header_size
            .try_into()
            .expect("Could not convert header size")
    ];
    file.read(&mut header_buffer).unwrap();

    let header = parse_from_bytes::<BlockHeader>(&header_buffer).expect("Header not valid");

    let mut blob_buffer = vec![
        0;
        header
            .get_datasize()
            .try_into()
            .expect("Could not convert buffer size")
    ];

    file.read(&mut blob_buffer).unwrap();

    let blob_data = parse_from_bytes::<Blob>(&blob_buffer).expect("Blob Data not valid");
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

    return Ok(());
}

fn load_proto_message<T: Message>(data: Vec<u8>) -> T {
    let header_block = parse_from_bytes::<T>(&data)
        .expect(format!("Could not load decompressed block with data {:?}", data).as_str());
    header_block
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
    let primitive_groups = block.get_primitivegroup().into_iter();
    println!("String Table = {:?}", &string_table.len());
    println!("Granularity = {:?}", granularity);
    println!("Date Granularity = {:?}", date_granularity);
    println!("Primitive Groups = {:?}", primitive_groups.len());
    for group in primitive_groups {
        handle_dense_nodes(group.get_dense(), &string_table);
        // println!("Dense: {:?}", );

        // println!("Nodes: {:?}", group.get_nodes());
        // println!("Ways: {:?}", group.get_ways());
        // println!("Relations: {:?}", group.get_relations());

        // nodes: ::protobuf::RepeatedField<Node>,
        // dense: ::protobuf::SingularPtrField<DenseNodes>,
        // ways: ::protobuf::RepeatedField<Way>,
        // relations: ::protobuf::RepeatedField<Relation>,
        // changesets: ::protobuf::RepeatedField<ChangeSet>,
    }
}

fn handle_dense_nodes(nodes: &DenseNodes, string_table: &Vec<&str>) {
    println!("Dense Info {:?}", nodes.get_id().len());
    let _ids = delta_decode(0, nodes.get_id());
    let _uids = delta_decode(0, nodes.get_denseinfo().get_uid());
    let _sids = delta_decode(0, nodes.get_denseinfo().get_user_sid());
    let _timestamps = delta_decode(0, nodes.get_denseinfo().get_timestamp());
    let _changesets = delta_decode(0, nodes.get_denseinfo().get_changeset());
    let _latitudes = delta_decode(0, nodes.get_lat());
    let _longitudes = delta_decode(0, nodes.get_lon());

    let key_vals = build_key_vals(nodes.get_keys_vals(), &string_table);
    println!("Key_Vals {:?}", key_vals);
}

fn build_key_vals(key_vals: &[i32], string_table: &Vec<&str>) -> Vec<(String, String)> {
    // let mut results = HashMap::new();
    let mut results: Vec<(String, String)> = vec![];
    for x in key_vals.chunks(2) {
        let key = string_table.get(x[0] as usize).unwrap();
        let val = string_table.get(x[1] as usize).unwrap();
        // results.insert(String::from(*key), String::from(*val));
        results.push((String::from(*key), String::from(*val)));
    }
    results
}

fn delta_decode<T>(seed: T, data: &[T]) -> Vec<T>
where
    T: Add<Output = T> + Copy + Display,
{
    let mut decoded: Vec<T> = vec![];
    let mut running_total = seed;
    for e in data.into_iter() {
        running_total = running_total + *e;
        decoded.push(running_total);
    }
    decoded
}

fn convert_string_table(string_table: &StringTable) -> Vec<&str> {
    string_table
        .get_s()
        .into_iter()
        .map(|x| str::from_utf8(x).unwrap())
        .collect()
}

fn get_datetime(timestamp: i64) -> DateTime<Utc> {
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    return DateTime::from_utc(naive, Utc);
}
