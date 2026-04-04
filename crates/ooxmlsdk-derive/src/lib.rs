use proc_macro::TokenStream;

#[proc_macro_derive(SdkEnum, attributes(sdk))]
pub fn sdk_enum(_input: TokenStream) -> TokenStream {
  TokenStream::new()
}

#[proc_macro_derive(SdkType, attributes(sdk))]
pub fn sdk_type(_input: TokenStream) -> TokenStream {
  TokenStream::new()
}

#[proc_macro_derive(SdkChoice, attributes(sdk))]
pub fn sdk_choice(_input: TokenStream) -> TokenStream {
  TokenStream::new()
}
