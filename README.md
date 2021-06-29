Wasm In-SPEC-tor 🔬
===================

A simple CLI tool to extract the [`RuntimeVersion`](https://crates.parity.io/sp_version/struct.RuntimeVersion.html) details including `spec_name` and `spec_version` from a Substrate Runtime's wasm blob.

```bash
# Generate, Download, or otherwise save a wasm file
moonbeam export-genesis-wasm --chain moonriver --raw > moonriver-genesis.wasm

# Use this tool to inspect it
cargo run moonriver-genesis.wasm
```