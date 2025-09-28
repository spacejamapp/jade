#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;

mod accumulate;
mod authorize;
mod refine;

/// Export the is_authorized interface
///
/// TODO: replace the function body directly
#[proc_macro_attribute]
pub fn is_authorized(args: TokenStream, input: TokenStream) -> TokenStream {
    authorize::is_authorized(args, input)
}

/// Export the refine interface
#[proc_macro_attribute]
pub fn refine(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// Export the accumulate interface
#[proc_macro_attribute]
pub fn accumulate(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
