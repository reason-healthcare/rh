fn main() {
    if std::env::var("CARGO_FEATURE_WASM").is_ok() {
        println!("cargo:rustc-cfg=wasm_feature");
    }
}
