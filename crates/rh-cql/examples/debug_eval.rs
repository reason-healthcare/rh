use rh_cql::compile_with_model;

fn main() {
    let cql = "library T define X: 3 + 4";
    let r = compile_with_model(cql, None, None).unwrap();
    eprintln!("errors: {:?}", r.errors);
    eprintln!("json: {}", r.to_compact_json().unwrap());
}
