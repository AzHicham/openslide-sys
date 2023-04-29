#[doc(hidden)]
#[allow(clippy::all)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub mod sys {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
