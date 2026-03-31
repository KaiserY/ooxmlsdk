#![cfg_attr(doc, recursion_limit = "512")]

pub mod common;
pub mod deserializers;
pub mod namespaces;
#[cfg(feature = "parts")]
pub mod parts;
pub mod schemas;
pub mod serializers;
pub mod simple_type;
