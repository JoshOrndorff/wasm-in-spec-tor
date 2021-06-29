use sc_executor::read_embedded_version;
use sc_executor_common::runtime_blob::RuntimeBlob;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    /// Interpret the input file's contents as hexadecimal
    #[structopt(long)]
    hex: bool,

    /// Path to the file containing the Runtime's wasm bytecode.
    filename: String,
}

fn main() {
    let Opt { hex, filename } = Opt::from_args();

    let file_contents = std::fs::read(filename).expect("Should be able to read the file");

    let code = if hex {
        hex_to_raw(&file_contents)
    } else {
        file_contents
    };

    let blob = RuntimeBlob::uncompress_if_needed(&code)
        .expect("wasm runtime blob should be built successfully");

    let version = read_embedded_version(&blob)
        .expect("Should be able to decode any custom wasm sections encountered")
        .expect("Custom wasm section containing the runtime version should be present");

    println!("spec name: {}", version.spec_name);
    println!("spec version: {}", version.spec_version);
    println!("impl version: {}", version.impl_version);
}

fn hex_to_raw(hex_input: &[u8]) -> Vec<u8> {
    let without_prefix = if hex_input[0..2] == "0x".to_owned().into_bytes() {
        &hex_input[2..]
    } else {
        hex_input
    };

    hex::decode(without_prefix)
        .expect("File should decode as hex when the `--hex` flag is passed")
}