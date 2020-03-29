extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;

#[proc_macro_attribute]
pub fn error_context(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let input = syn::parse_macro_input!(input as syn::ItemFn);

    let fmt = match args.as_slice() {
        [syn::NestedMeta::Lit(syn::Lit::Str(fmt))] => fmt,
        _ => panic!("expected single string literal argument"),
    };

    let inner_fn = syn::ItemFn {
        attrs: input.attrs,
        vis: syn::Visibility::Inherited,
        sig: syn::Signature {
            ident: syn::Ident::new("inner", Span::call_site()),
            ..input.sig.clone()
        },
        block: input.block,
    };

    let outer_fn = syn::ItemFn {
        attrs: vec![],
        vis: input.vis,
        sig: input.sig,
        block: syn::parse(
            quote!({
                #inner_fn
                inner().map_err(|err| err.context(format!(#fmt)))
            })
            .into(),
        )
        .unwrap(),
    };

    quote!(#outer_fn).into()
}
