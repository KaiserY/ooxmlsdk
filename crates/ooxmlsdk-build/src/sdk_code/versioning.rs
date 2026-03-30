use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, parse_quote};

pub fn version_cfg_attrs(version: &str) -> Vec<Attribute> {
  if is_microsoft365_version(version) {
    vec![parse_quote!(#[cfg(feature = "microsoft365")])]
  } else {
    Vec::new()
  }
}

pub fn not_microsoft365_cfg_attrs() -> Vec<Attribute> {
  vec![parse_quote!(#[cfg(not(feature = "microsoft365"))])]
}

pub fn features_cfg_attrs(features: &[String]) -> Vec<Attribute> {
  let mut attrs = Vec::new();

  if features.iter().any(|feature| feature == "parts") {
    attrs.push(parse_quote!(#[cfg(feature = "parts")]));
  }

  if features.iter().any(|feature| feature == "microsoft365") {
    attrs.push(parse_quote!(#[cfg(feature = "microsoft365")]));
  }

  attrs
}

pub fn versioned_tokens(version: &str, tokens: TokenStream) -> TokenStream {
  let attrs = version_cfg_attrs(version);

  quote! {
    #( #attrs )*
    #tokens
  }
}

pub fn is_microsoft365_version(version: &str) -> bool {
  matches!(
    version,
    "Office2010" | "Office2013" | "Office2016" | "Office2019" | "Office2021" | "Microsoft365"
  )
}

pub fn effective_version<'a>(left: &'a str, right: &'a str) -> &'a str {
  if is_microsoft365_version(left) {
    left
  } else if is_microsoft365_version(right) {
    right
  } else if !left.is_empty() {
    left
  } else {
    right
  }
}
