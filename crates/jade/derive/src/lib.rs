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
pub fn refine(args: TokenStream, input: TokenStream) -> TokenStream {
    refine::refine(args, input)
}

/// Export the accumulate interface
#[proc_macro_attribute]
pub fn accumulate(args: TokenStream, input: TokenStream) -> TokenStream {
    accumulate::accumulate(args, input)
}
