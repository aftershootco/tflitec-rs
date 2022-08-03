use std::{
    env,
    path::PathBuf,
    // process::Command,
};

extern crate bindgen;

fn out_dir() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

fn main() {
    // panic!();
    let out_path = out_dir();
    let mut builder = bindgen::Builder::default().header("include/c_api.h");
    if cfg!(feature = "xnnpack") {
        builder = builder.header("include/xnnpack_delegate.h");
    }

    let bindings = builder
        .clang_arg("-Iinclude")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
