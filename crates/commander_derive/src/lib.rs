use proc_macro::TokenStream;
use syn::parse_macro_input;

mod api_response;
mod utils;

use api_response::impl_api_response;

#[proc_macro_derive(ApiResponse, attributes(response, from))]
pub fn api_response(input: TokenStream) -> TokenStream {
    impl_api_response(parse_macro_input!(input)).into()
}
