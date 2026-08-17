use std::{env, path::Path};

const LIB_OPENSLIDE: &str = "openslide";

fn main() {
    let dynamic_link = cfg!(feature = "dynamic-link");

    let library = pkg_config::Config::new()
        .statik(!dynamic_link)
        .probe(LIB_OPENSLIDE)
        .unwrap_or_else(|err| panic!("failed to find {LIB_OPENSLIDE} via pkg-config: {err}"));

    let include_dir = library
        .include_paths
        .first()
        .expect("pkg-config returned no include path for openslide");

    let bindings = bindgen::Builder::default()
        .header(include_dir.join("openslide.h").to_string_lossy())
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_dir = env::var_os("OUT_DIR").expect("Var env OUT_DIR is undefined");
    let dest_path = Path::new(&out_dir).join("bindings.rs");
    bindings
        .write_to_file(dest_path)
        .expect("Couldn't write bindings!");
}
