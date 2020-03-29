// #![deny(missing_docs)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn context(args: TokenStream, input: TokenStream) -> TokenStream {
    let args: proc_macro2::TokenStream = args.into();
    let mut input = syn::parse_macro_input!(input as syn::ItemFn);

    let body = &input.block;
    let return_ty = &input.sig.output;
    input.block = syn::parse_quote!({
        (|| #return_ty #body)().map_err(|err| err.context(format!(#args)).into())
    });

    quote!(#input).into()
}
