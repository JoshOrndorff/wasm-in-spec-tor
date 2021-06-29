use sc_executor::read_embedded_version;
use sc_executor_common::runtime_blob::RuntimeBlob;

fn main() {
    println!("Hello, world!");

    // The code needs to be in "raw" format, not hex.
    // TODO find a way to convert if hex is provided.
    let code = &include_bytes!("../moonbase.wasm")[..];

    let blob = RuntimeBlob::uncompress_if_needed(code)
        .expect("wasm runtime blob should be built successfully");

    let version = read_embedded_version(&blob)
        .expect("Should be able to decode any custom wasm sections encountered")
        .expect("Custom wasm section containing the runtime version should be present");

    assert_eq!(version.spec_name, "moonbase".into());

    assert_eq!(version.spec_version, 53);
}
