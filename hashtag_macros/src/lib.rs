#![doc = "include_str!(\"../README.md\")"]
// ref: https://earthly.dev/blog/rust-macros/

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, ItemFn};

#[proc_macro_attribute]
/// Attribute macro for tagging test function with custom hastags.
///
/// This macro allows users to associate one or more tags (seperated by commas) with a test function.
/// It modifies the function by adding documetation and registering the test with the provided tags
/// for later selective running.
///
/// # Arguments
///
/// * `attr` - A `proc_macro::TokenStream` containing the tags provided to the macro.
/// * `input` - A `proc_macro::TokenStream` containing the function to be tagged.
///
/// # Returns
///
/// A `proc_macro::TokenStream` containing the modified function with added documentation
/// and registration with the provided tags.
///
/// # Functionality
///
/// 1. Parses the input function and extracts its name.
/// 2. Extracts tags from the attribute arguments.
/// 3. Adds a doc comment to the function with the tags.
/// 4. Creates a hidden constant for compile-time tag verification.
/// 5. Registers the function with its tags using the `inventory::submit!` macro.
/// # Example Usage
///
/// ```
/// #[hashtag("integration", "database")]
/// fn test_database_connection() {
///     // Test implementation...
/// }
/// ```
///
/// # Notes
///
/// - This macro is designed to work with the `inventory` crate for test collection.
/// - The added hidden constant allows for compile-time verification of tags.
pub fn hashtag(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemFn);
    let fn_name = &input.sig.ident;

    let tags: Vec<String> = attr
        .into_iter()
        .filter_map(|token| {
            if let proc_macro::TokenTree::Literal(lit) = token {
                Some(lit.to_string().trim_matches('"').to_string())
            } else {
                None
            }
        })
        .collect();

    let tags_str = tags.join(", ");

    let doc_attr: Attribute = syn::parse_quote!(#[doc = concat!("Tags: ", #tags_str)]);
    input.attrs.push(doc_attr);

    let output = quote! {
        // This attribute tells the Rust compiler to not warn anout this code being unused.
        // it's necessary because this constant is not directly used in the code,
        // but is used for compile-time verification of the tags.
        #[allow(dead_code)]
        // `doc(hidden)` hides this item from the generated documentation.
        // we don't want this implementation detail to show up in the user-facing documentation.
        #[doc(hidden)]
        // not really necessary. TODO: consider removing this attribute later.
        #[must_use = "Hashtag function should be called"]
        // create an anonymous constant of type () (uint type).
        // This cannot be referred to later in the code, but it is used to verify the tags.
        const _: () = {
            // holds the actual tags as a string.
            // `#tags_str` is replaced with the actual tags provided to the macro/
            const _HASHTAG_TAGS: &str = #tags_str;
        };

        #input

        inventory::submit! {
            hashtag_core::TaggedTest {
                name: stringify!(#fn_name),
                tags: &[#(#tags),*],
                test_fn: #fn_name,
            }
        }
    };

    output.into()
}
