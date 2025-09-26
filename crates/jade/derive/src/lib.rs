#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod accumulate;
mod authorize;
mod refine;

/// Export the is_authorized interface
///
/// TODO: replace the function body directly
#[proc_macro_attribute]
pub fn is_authorized(_args: TokenStream, input: TokenStream) -> TokenStream {
    let fun = parse_macro_input!(input as syn::ItemFn);
    let funame = fun.sig.ident.clone();

    // construct the export
    let export = quote::quote! {
        extern "C" fn jade_is_authorized(ptr: u32, size: u32) -> (u64, u64) {
            let mut buf = [0; size as usize];
            unsafe { buf.copy_from_slice(core::slice::from_raw_parts(ptr as *const u8, size as usize)); }
            let (param, package, core_index): (AuthConfig, WorkPackage, CoreIndex) =
                codec::decode(buf).expect("failed to decode");
            let result = #funame(param, package, core_index);
            ((&result).as_ptr() as u64, result.len() as u64)
        }
    };

    quote::quote!(
        #fun

        #export
    )
    .into()
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
