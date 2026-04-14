#![cfg_attr(doc, recursion_limit = "512")]

pub mod common;
pub mod namespaces;
#[cfg(feature = "parts")]
pub mod parts;
pub mod schemas;
pub mod sdk;
pub mod simple_type;
pub mod validator;
