use proc_macro::TokenStream;

#[proc_macro_derive(SdkEnum, attributes(sdk))]
pub fn sdk_enum(_input: TokenStream) -> TokenStream {
  TokenStream::new()
}
