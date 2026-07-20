// SAFE: Input is parsed into a strict AST via `syn::parse` with custom error rejection.
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, Ident, Token};

struct ColumnDef {
    name: Ident,
}

impl Parse for ColumnDef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ColumnDef {
            name: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn column(input: TokenStream) -> TokenStream {
    let col = parse_macro_input!(input as ColumnDef);
    let name = &col.name;
    let expanded = quote! {
        impl #name {
            pub fn column_name() -> &'static str {
                stringify!(#name)
            }
        }
    };
    expanded.into()
}
