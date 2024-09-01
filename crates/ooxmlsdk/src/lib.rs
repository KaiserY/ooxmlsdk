#![recursion_limit = "256"]

pub mod schemas {
  include!(concat!(env!("OUT_DIR"), "/schemas/mod.rs"));
}
