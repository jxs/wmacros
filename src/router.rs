use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn, Expr};

pub(crate) fn route(method: &str, attr: TokenStream, item: TokenStream) -> TokenStream {
    let location: Expr = parse_macro_input!(attr);
    let item: ItemFn = parse_macro_input!(item);

    let (existing_fn_name, existing_fn_output, existing_fn_inputs, existing_fn_block) = (
        item.sig.ident,
        item.sig.output,
        item.sig.inputs,
        item.block,
    );

    let fn_name = existing_fn_name.clone();
    let existing_new_name = format_ident!("{}_{}", method, existing_fn_name);

    let old_renamed = quote! {async fn #existing_new_name(#existing_fn_inputs) #existing_fn_output #existing_fn_block};

    let result = quote! {
        use warp::Filter;
        #old_renamed

        fn #fn_name() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
            warp::path!(#location)
                .and(warp::get())
                .and_then(#existing_new_name)
        }
    };

    result.into()

}
