//! This crate provides the [`context`](attr.context.html) macro for adding extra error
//! information to a function.
//!
//! Works with [`anyhow`], [`failure`] and any other error type which
//! provides a `context` method taking a string.
//!
//! ```
//! # use std::fs::read_to_string;
//! # use std::path::Path;
//! #
//! use fn_error_context::context;
//!
//! #[context("failed to parse config at `{}`", path.as_ref().display())]
//! pub fn parse_config(path: impl AsRef<Path>) -> anyhow::Result<u32> {
//!     let text = read_to_string(path.as_ref())?;
//!     Ok(text.parse()?)
//! }
//!
//! assert_eq!(
//!     parse_config("not-found").unwrap_err().to_string(),
//!     "failed to parse config at `not-found`"
//! );
//! ```
//!
//! [`anyhow`]: https://crates.io/crates/anyhow
//! [`failure`]: https://crates.io/crates/failure

#![doc(html_root_url = "https://docs.rs/fn-error-context/0.1.0")]
#![deny(missing_docs)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

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
