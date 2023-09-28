// ref: https://earthly.dev/blog/rust-macros/

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

mod test;

#[proc_macro_attribute]
pub fn hashtag(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    println!("input: {:?}", input.sig.ident);
    println!("args: {:?}", _attr.to_string());
    TokenStream::from(quote!(#input))
}

#[cfg(test)]
mod attribute_macro_tests {
    use super::*;

    fn parse_attributes<S: AsRef<&str>>(attr: S) {
        todo!()
    }
}