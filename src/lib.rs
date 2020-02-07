extern crate proc_macro;

mod router;

use proc_macro::TokenStream;
use router::route;

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn GET(attr: TokenStream, item: TokenStream) -> TokenStream {
    route("get", attr, item)
}

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn POST(attr: TokenStream, item: TokenStream) -> TokenStream {
    route("post", attr, item)
}
