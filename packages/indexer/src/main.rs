#![allow(dead_code)]
mod index;
use index::data_types::Indexable;

mod protos;
use protos::fileformat::{Blob, BlockHeader};
use protos::osmformat::{HeaderBlock, PrimitiveBlock};

use protobuf::{parse_from_bytes, Message};
use std::convert::TryInto;
use std::env;
use std::fs::{self};
use std::io;
use std::io::prelude::*;
use std::path::Path;

use flate2::{Decompress, FlushDecompress};

fn main() {
    println!("--- Start ---");
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let query_path = &args[1];
    let path = Path::new(query_path);
    start_import(path);
    // visit_dirs(&path).unwrap();
    println!("--- End ---");
}

fn start_import(path: &Path) {
    match path.is_file() {
        true => {
            let mut data_file = fs::File::open(&path).expect("File does not exist");
            // Header
            get_block(&mut data_file);
            // Data
            get_block(&mut data_file);   
        }
        false => panic!("This is not a file")
    }
}

fn get_block(file: &mut fs::File) {
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
        println!("Header Block {:?}", result);
    } else {
        let result = load_proto_message::<PrimitiveBlock>(uncompressed_blob_buffer);
        println!("Data Block {:?}", result);
    }
}

fn load_proto_message<T: Message>(data: Vec<u8>) -> T {
    let header_block = parse_from_bytes::<T>(&data)
        .expect(format!("Could not load decompressed block with data {:?}", data).as_str());
    header_block
}




// ------- OLD CODE ---------
fn visit_dirs(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            match path.is_dir() {
                true => visit_dirs(&path)?,
                false => println!("{}", index_file(entry)?.information()),
            }
        }
    }
    Ok(())
}

fn index_file(entry: fs::DirEntry) -> io::Result<Indexable<String>> {
    let file_name = String::from(entry.file_name().to_str().unwrap());
    let modified = entry.metadata()?.created()?;
    let index = Indexable {
        name: file_name,
        modified: modified,
        is_file: entry.file_type()?.is_file(),
        is_dir: entry.file_type()?.is_dir(),
        is_symlink: entry.file_type()?.is_symlink(),
    };

    return Ok(index);
}
