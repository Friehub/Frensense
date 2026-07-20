// [frensense]
// observation: A procedural macro converts its entire input `TokenStream` to a string and passes it through to `quote!` or `syn::parse_str` without any validation or sanitization, allowing crafted input to inject arbitrary Rust tokens.
// impact: A malicious user of the macro can inject arbitrary Rust code that compiles in the context of the caller's crate, leading to arbitrary code execution at compile time or runtime.
// improvement: Parse the input into a well-defined AST with `syn::parse_macro_input!` and reject unexpected tokens rather than interpolating raw strings.

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn sql_query(input: TokenStream) -> TokenStream {
    let raw = input.to_string();
    let expanded = quote! {
        unsafe { #raw }
    };
    expanded.into()
}

#[proc_macro_derive(FromJson)]
pub fn from_json(input: TokenStream) -> TokenStream {
    let raw = input.to_string();
    let expanded = quote! {
        impl From<String> for #raw {
            fn from(s: String) -> Self { serde_json::from_str(&s).unwrap() }
        }
    };
    expanded.into()
}
