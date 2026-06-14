use vergen::EmitBuilder;

fn main() {
    EmitBuilder::builder()
        .build_timestamp()
        .git_sha(true)
        .git_describe(true, true, None)
        .emit()
        .expect("vergen failed");
}
