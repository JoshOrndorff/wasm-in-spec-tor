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

    let code = std::fs::read(filename).expect("Should be able to read the file");

    let blob = RuntimeBlob::uncompress_if_needed(&code)
        .expect("wasm runtime blob should be built successfully");

    let version = read_embedded_version(&blob)
        .expect("Should be able to decode any custom wasm sections encountered")
        .expect("Custom wasm section containing the runtime version should be present");

    println!("spec name: {}", version.spec_name);
    println!("spec version: {}", version.spec_version);
    println!("impl version: {}", version.impl_version);
}
