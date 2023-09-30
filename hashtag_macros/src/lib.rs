// ref: https://earthly.dev/blog/rust-macros/

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn hashtag(attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemFn);
    let _fn_name = ast.sig.ident.to_owned();

    let tags = attr.to_string();
    let _tags_vec = string_to_vec(tags);
    TokenStream::from(quote!(#ast))
}

fn string_to_vec(tags: String) -> Vec<String> {
    let tags_vec = tags
        .split(',')
        .map(|s| s.trim().replace('"', "").to_owned())
        .collect::<Vec<String>>();

    tags_vec
}
