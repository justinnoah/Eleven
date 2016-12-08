fn main() {
    extern crate serde_codegen;

    use std::env;
    use std::path::Path;

    let out_dir = env::var_os("OUT_DIR").unwrap();

    // JSON Serialization
    let json_src = Path::new("src/data_types.in.rs");
    let json_dst = Path::new(&out_dir).join("data_types.rs");
    serde_codegen::expand(&json_src, &json_dst).unwrap();

    // SQL Serialization
    let sql_src = Path::new("src/db/models.in.rs");
    let sql_dst = Path::new(&out_dir).join("models.rs");
    serde_codegen::expand(&sql_src, &sql_dst).unwrap();
}
