use proc_macro2::TokenTree;
use syn::{Ident, parse::Parse};

struct Outer<T>(T);
impl<T> Parse for Outer<T>
where
    T: Parse,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let outer: Ident = input.parse()?;

        if outer == "outer" {
            let content;
            let _ = syn::parenthesized!(content in input);
            return content.parse().map(Outer);
        }

        Err(syn::Error::new(outer.span(), "expected `outer`"))
    }
}

pub fn parse_metadata<T, S>(test: S) -> T
where
    T: syn::parse::Parse,
    S: AsRef<str>,
{
    let to_parse = format!(
        r#"
        #[outer({})]
        fn to_parse() {{}}
        "#,
        test.as_ref()
    );

    let item_fn = syn::parse_str::<syn::ItemFn>(&to_parse)
        .expect(&format!("failed to parse test function: {}", to_parse));

    let tokens = quote::quote!(#item_fn);
    let token_tree = tokens.into_iter().next().unwrap();

    if let TokenTree::Group(g) = token_tree {
        let ts = g.stream();
        syn::parse2::<Outer<T>>(ts).unwrap().0
    } else {
        panic!("Cannot find group in {:#?}", token_tree)
    }
}
