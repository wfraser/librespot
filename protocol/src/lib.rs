#![allow(rust_2018_idioms)] // because of generated code

// This file is parsed by build.rs
// Each included module will be compiled from the matching .proto definition.

include!(concat!(env!("OUT_DIR"), "/mod.rs"));
