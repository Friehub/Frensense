// SAFE: Used a whitelist of allowed identifiers in proc-macro input, rejecting any arbitrary strings.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, LitStr};

const ALLOWED_GENERATORS: &[&str] = &["index", "show", "create", "update", "delete"];

#[proc_macro]
pub fn generate_handler(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Ident);
    let name_str = name.to_string();
    if !ALLOWED_GENERATORS.contains(&name_str.as_str()) {
        return syn::Error::new(name.span(), "Unknown handler").to_compile_error().into();
    }
    let expanded = quote! {
        pub fn #name(req: Request) -> Response {
            Response::json("ok")
        }
    };
    expanded.into()
}
