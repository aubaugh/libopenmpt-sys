fn main () {
    if cfg!(feature = "generate-bindings") {
        use std::fs::copy;
        use std::path::PathBuf;
        let bindings_path = concat!(env!("OUT_DIR"), "/bindings.rs");
        copy(PathBuf::from(bindings_path), PathBuf::from("src/bindings.rs")).unwrap();
    } else {
        panic!("Must be run with 'generate-bindings' feature");
    }
}
