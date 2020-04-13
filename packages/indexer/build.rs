use std::fs;

static PROTOCOL_BUFFER_DIRECTORY: &str = "src/protos";

fn main() {
    if fs::create_dir(PROTOCOL_BUFFER_DIRECTORY).is_err() {
        println!("Folder {} already exists", PROTOCOL_BUFFER_DIRECTORY);
    }

    protobuf_codegen_pure::Codegen::new()
        .out_dir("src/protos")
        .inputs(&["protos/fileformat.proto", "protos/osmformat.proto"])
        .include("protos")
        .run()
        .expect("Codegen failed.");
}