extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;

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

    let (inputs, params) = remove_patterns(&input.sig.inputs);

    let outer_fn = syn::ItemFn {
        attrs: vec![],
        vis: input.vis,
        sig: syn::Signature {
            inputs,
            ..input.sig
        },
        block: syn::parse(
            quote!({
                #inner_fn
                inner(#params).map_err(|err| err.context(format!(#fmt)))
            })
            .into(),
        )
        .unwrap(),
    };

    quote!(#outer_fn).into()
}

fn remove_patterns(
    inputs: &Punctuated<syn::FnArg, Comma>,
) -> (Punctuated<syn::FnArg, Comma>, Punctuated<syn::Expr, Comma>) {
    inputs
        .iter()
        .enumerate()
        .map(|(n, arg)| match arg {
            syn::FnArg::Receiver(_) => panic!("inherent methods are not supported"),
            syn::FnArg::Typed(pat_type) => {
                let ident = match &*pat_type.pat {
                    syn::Pat::Ident(pat) => pat.ident.clone(),
                    _ => syn::Ident::new(&format!("arg{}", n), Span::call_site()),
                };
                (
                    syn::FnArg::Typed(syn::PatType {
                        pat: Box::new(syn::Pat::Ident(syn::PatIdent {
                            attrs: vec![],
                            mutability: None,
                            by_ref: None,
                            ident: ident.clone(),
                            subpat: None,
                        })),
                        ..pat_type.clone()
                    }),
                    syn::Expr::Path(syn::ExprPath {
                        attrs: vec![],
                        qself: None,
                        path: ident.into(),
                    }),
                )
            }
        })
        .unzip()
}
