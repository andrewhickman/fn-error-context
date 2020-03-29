extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;

#[proc_macro_attribute]
pub fn error_context(args: TokenStream, input: TokenStream) -> TokenStream {
    let Args { fmt, args } = syn::parse_macro_input!(args as Args);
    let mut input = syn::parse_macro_input!(input as syn::ItemFn);

    let body = &input.block;
    let return_ty = &input.sig.output;
    input.block = syn::parse_quote!({
        (|| #return_ty #body)().map_err(|err| err.context(format!(#fmt, #args)).into())
    });

    quote!(#input).into()
}

struct Args {
    fmt: syn::LitStr,
    args: Punctuated<syn::Expr, Comma>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fmt = input.parse()?;
        let args = if let Some(_) = input.parse::<Option<Comma>>()? {
            Punctuated::parse_separated_nonempty(input)?
        } else {
            Punctuated::new()
        };

        Ok(Args { fmt, args })
    }
}
