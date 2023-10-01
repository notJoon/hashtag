// ref: https://earthly.dev/blog/rust-macros/

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn hashtag(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemFn);
    // let _fn_name = ast.sig.ident.to_owned();
    // let tags_str = _attr.to_string();
    // let _tag_vec = tags_str.split(',').map(|s| s.trim().to_string()).collect::<Vec<String>>();
    TokenStream::from(quote!(#ast))
}
