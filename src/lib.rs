pub extern crate libc;
extern crate qt_core;
extern crate cpp_core;

#[allow(dead_code)]
mod ffi {
  include!(concat!(env!("OUT_DIR"), "/ffi.rs"));
}

pub mod custom_event_filter;
