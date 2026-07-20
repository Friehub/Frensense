// SAFE: Macro input is parsed through `syn::parse_macro_input!` and validated.
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, LitStr};

#[proc_macro]
pub fn sql_query(input: TokenStream) -> TokenStream {
    let table = parse_macro_input!(input as Ident);
    let expanded = quote! {
        format!("SELECT * FROM {}", stringify!(#table))
    };
    expanded.into()
}

#[proc_macro_derive(FromJson)]
pub fn from_json(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;
    let expanded = quote! {
        impl From<String> for #name {
            fn from(s: String) -> Self { serde_json::from_str(&s).unwrap() }
        }
    };
    expanded.into()
}
