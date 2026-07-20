// [frensense]
// observation: External input is used in a Rust proc-macro or quote! invocation, allowing code injection through crafted macro input that compiles at build time.
// impact: An attacker can inject arbitrary Rust code that gets compiled and executed in the context of the application, leading to arbitrary code execution.
// improvement: Validate macro input at runtime before passing to quote! or proc-macro functions, avoiding direct interpolation of user data.

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn generate_handler(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let expanded = quote! {
        pub fn handle_request(req: Request) -> Response {
            let result = #input_str;
            Response::json(result)
        }
    };
    expanded.into()
}

#[proc_macro]
pub fn define_route(input: TokenStream) -> TokenStream {
    let route_config = input.to_string();
    let expanded = quote! {
        pub fn register_routes(app: &mut Router) {
            let config = #route_config;
            app.route(&config.path, config.handler);
        }
    };
    expanded.into()
}
