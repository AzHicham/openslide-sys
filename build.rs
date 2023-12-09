use std::{env, path::Path};

#[cfg(feature = "dynamic-link")]
fn statik_link() -> bool {
    false
}

#[cfg(not(feature = "dynamic-link"))]
fn statik_link() -> bool {
    true
}

fn probe(s: &str) -> pkg_config::Library {
    pkg_config::Config::new()
        .cargo_metadata(false)
        .probe(s)
        .unwrap()
}

fn link_library(s: &str) {
    pkg_config::Config::new()
        .statik(statik_link())
        .probe(s)
        .unwrap();
}

const LIB_OPENSLIDE: &str = "openslide";

fn main() {
    let libopenslide = probe(LIB_OPENSLIDE);

    let include_dir = libopenslide
        .include_paths
        .get(0)
        .expect("Could not find include path for openslide");

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

    link_library(LIB_OPENSLIDE);
}
