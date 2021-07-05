//! This crate provides the [`context`](attr.context.html) macro for adding extra error
//! information to a function.
//!
//! Works with [`anyhow`], [`failure`] and any other error type which
//! provides a `context` method taking a string.
//!
//! ```
//! # use std::fs::read_to_string;
//! # use std::path::Path;
//! # use std::io;
//! #
//! use fn_error_context::context;
//!
//! #[context("failed to parse config at `{}`", path.as_ref().display())]
//! pub fn parse_config(path: impl AsRef<Path>) -> anyhow::Result<u32> {
//!     let text = read_to_string(path.as_ref())?;
//!     Ok(text.parse()?)
//! }
//!
//! let error = parse_config("not-found").unwrap_err();
//! assert_eq!(
//!     error.to_string(),
//!     "failed to parse config at `not-found`",
//! );
//! assert_eq!(
//!     error.source().unwrap().downcast_ref::<io::Error>().unwrap().kind(),
//!     io::ErrorKind::NotFound,
//! );
//! ```
//!
//! [`anyhow`]: https://crates.io/crates/anyhow
//! [`failure`]: https://crates.io/crates/failure

#![doc(html_root_url = "https://docs.rs/fn-error-context/0.1.2")]
#![deny(missing_docs)]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::parse::{self, Parse, ParseStream};
use syn::Token;

/// Add context to errors from a function.
///
/// The arguments to this macro are a format string with arguments using
/// the standard `std::fmt` syntax. Arguments to the function can be used
/// as long as they are not consumed in the function body.
///
/// This macro desugars to something like
/// ```
/// # fn function_body() -> anyhow::Result<()> { unimplemented!() }
/// #
/// pub fn function() -> anyhow::Result<()> {
///     (|| -> anyhow::Result<()> {
///         function_body()
///     })().map_err(|err| err.context("context"))
/// }
/// ```
///
/// Sometimes you will receive borrowck errors, especially when returning references. These can
/// often be fixed by setting the `move` option of the attribute macro. For example:
///
/// ```
/// use fn_error_context::context;
///
/// #[context(move, "context")]
/// fn returns_reference(val: &mut u32) -> anyhow::Result<&mut u32> {
///     Ok(&mut *val)
/// }
/// ```
#[proc_macro_attribute]
pub fn context(args: TokenStream, input: TokenStream) -> TokenStream {
    let Args(move_token, format_args) = syn::parse_macro_input!(args);
    let mut input = syn::parse_macro_input!(input as syn::ItemFn);

    let body = &input.block;
    let return_ty = &input.sig.output;
    let err = Ident::new("err", Span::mixed_site());
    let new_body = if input.sig.asyncness.is_some() {
        let return_ty = match return_ty {
            syn::ReturnType::Default => {
                return syn::Error::new_spanned(input, "function should return Result")
                    .to_compile_error()
                    .into();
            }
            syn::ReturnType::Type(_, return_ty) => return_ty,
        };
        let result = Ident::new("result", Span::mixed_site());
        quote! {
            let #result: #return_ty = async #move_token { #body }.await;
            #result.map_err(|#err| #err.context(format!(#format_args)).into())
        }
    } else {
        let force_fn_once = Ident::new("force_fn_once", Span::mixed_site());
        quote! {
            // Moving a non-`Copy` value into the closure tells borrowck to always treat the closure
            // as a `FnOnce`, preventing some borrowing errors.
            let #force_fn_once = ::core::iter::empty::<()>();
            (#move_token || #return_ty {
                ::core::mem::drop(#force_fn_once);
                #body
            })().map_err(|#err| #err.context(format!(#format_args)).into())
        }
    };
    input.block.stmts = vec![syn::Stmt::Expr(syn::Expr::Verbatim(new_body))];

    input.into_token_stream().into()
}

struct Args(Option<Token![move]>, TokenStream2);
impl Parse for Args {
    fn parse(input: ParseStream<'_>) -> parse::Result<Self> {
        let move_token = if input.peek(Token![move]) {
            let token = input.parse()?;
            input.parse::<Token![,]>()?;
            Some(token)
        } else {
            None
        };
        Ok(Self(move_token, input.parse()?))
    }
}
