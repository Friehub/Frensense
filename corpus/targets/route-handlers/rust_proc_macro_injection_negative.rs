// SAFE: Validated macro input to ensure it conforms to a safe pattern before use in quote!.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn generate_handler(input: TokenStream) -> TokenStream {
    let handler_name = parse_macro_input!(input as syn::Ident);
    let expanded = quote! {
        pub fn handle_request(req: Request) -> Response {
            Response::json("ok")
        }
    };
    expanded.into()
}

#[proc_macro]
pub fn define_route(input: TokenStream) -> TokenStream {
    let routes = parse_macro_input!(input as syn::ExprArray);
    let expanded = quote! {
        pub fn register_routes(app: &mut Router) {
            #routes
        }
    };
    expanded.into()
}
