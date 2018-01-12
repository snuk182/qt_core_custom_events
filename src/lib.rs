pub extern crate libc;
pub extern crate cpp_utils;
extern crate qt_core;

#[allow(dead_code)]
mod ffi {
  include!(concat!(env!("OUT_DIR"), "/ffi.rs"));
}

mod type_sizes {
  include!(concat!(env!("OUT_DIR"), "/type_sizes.rs"));
}

pub mod custom_event_filter;
