use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use std::error::Error;
use syn::{parse_str, Ident, Type};

use crate::models::OpenXmlSchema;
use crate::GenContext;

pub fn gen_deserializer(schema: &OpenXmlSchema, context: &GenContext) -> TokenStream {
  quote! {}
}
