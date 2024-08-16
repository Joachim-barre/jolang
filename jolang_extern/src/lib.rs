use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn jolang_extern(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input(input as ItemFn);

}
