use std::{env, path::Path};

fn probe(s: &str) -> pkg_config::Library {
    pkg_config::Config::new()
        .cargo_metadata(false)
        .probe(s)
        .unwrap()
}

fn link_library(s: &str) {
    pkg_config::Config::new().statik(true).probe(s).unwrap();
}

fn main() {
    let bindings = bindgen::Builder::default()
        .header("src/openslide-sys/openslide.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("bindings.rs");
    bindings
        .write_to_file(dest_path)
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=openslide");
}
