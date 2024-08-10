extern crate pkg_config;

#[cfg(feature = "generate-bindings")]
extern crate bindgen;

fn main() {
    match pkg_config::Config::new().statik(false).probe("libopenmpt") {
        Err(pkg_config::Error::Failure { command, output }) => panic!(
            "Pkg-config failed - usually this is because libopenmpt development headers are not installed.\n\n\
            pkg_config details:\n{}\n", pkg_config::Error::Failure { command, output }),
        Err(e) => panic!("{}", e),
        Ok(_library) => {
            #[cfg(feature = "generate-bindings")]
            generate_bindings(&_library);
        } 
    };
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings(library: &pkg_config::Library) {
    use std::env;
    use std::path::PathBuf;

    let clang_include_args = library.include_paths.iter().map(|include_path| {
        format!(
            "-I{}",
            include_path.to_str().expect("include path was not UTF-8")
        )
    });

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindgen::Builder::default()
        .clang_args(clang_include_args)
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .formatter(bindgen::Formatter::Prettyplease)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
