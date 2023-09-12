use std::env;

const HEADER: &str = "/*
* This file is automatically generated upon running
* `RUST_GENERATE_BINDINGS=1 cargo +nightly build`. Do not modify by hand.
*/";

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    if env::var_os("RUST_GENERATE_BINDINGS").is_none() {
        return;
    }

    cbindgen::Builder::new()
        .with_crate(crate_dir.clone())
        .with_no_includes()
        .with_sys_include("stdint.h")
        .with_parse_expand(&["rust-hashes"])
        .with_language(cbindgen::Language::C)
        .with_cpp_compat(true)
        .with_header(HEADER)
        .generate()
        .expect("failed to generate C bindings")
        .write_to_file("rust_hashes.h");
}