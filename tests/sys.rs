//! Smoke tests exercising the raw generated bindings directly, to catch a
//! bindgen/link mismatch that unit tests inside the crate wouldn't (this crate has
//! no unit tests: it's pure `extern "C"` declarations, nothing to unit-test).

use std::ffi::{CStr, CString};

use openslide_sys::sys;

#[test]
fn get_version_returns_a_non_empty_string() {
    let version = unsafe { sys::openslide_get_version() };
    assert!(!version.is_null(), "openslide_get_version returned NULL");

    let version = unsafe { CStr::from_ptr(version) }
        .to_str()
        .expect("version string is not valid UTF-8");
    assert!(!version.is_empty());
}

#[test]
fn detect_vendor_returns_null_for_a_nonexistent_file() {
    let path = CString::new("/nonexistent/path/to/a/slide.svs").unwrap();
    let vendor = unsafe { sys::openslide_detect_vendor(path.as_ptr()) };
    assert!(vendor.is_null());
}

#[test]
fn open_returns_null_for_a_nonexistent_file() {
    let path = CString::new("/nonexistent/path/to/a/slide.svs").unwrap();
    let slide = unsafe { sys::openslide_open(path.as_ptr()) };
    assert!(slide.is_null());
}
