// ref: https://earthly.dev/blog/rust-macros/

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn hashtag(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    println!("input: {:?}", input.sig.ident);
    println!("tags: {:?}", _attr.to_string());
    TokenStream::from(quote!(#input))
}
