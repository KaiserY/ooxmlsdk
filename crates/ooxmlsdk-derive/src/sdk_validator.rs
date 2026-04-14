use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn expand_sdk_validator(input: &DeriveInput) -> syn::Result<TokenStream> {
  let ident = &input.ident;
  let generics = &input.generics;
  let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

  Ok(quote! {
    impl #impl_generics crate::validator::SdkValidator for #ident #ty_generics #where_clause {}
  })
}
