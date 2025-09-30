//! authorize interface impl

use proc_macro::TokenStream;
use syn::{ItemFn, parse_macro_input, parse_quote};

/// Implement the is_authorized interface
///
/// 1. wrap the function with a C-compatible function
/// 2. impl with polkavm-derive-impl
pub fn accumulate(args: TokenStream, input: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(args as polkavm_derive_impl::ExportBlockAttributes);
    let fun = parse_macro_input!(input as syn::ItemFn);
    let funame = fun.sig.ident.clone();

    // construct the export
    //
    // TODO: introduce params check
    let export: ItemFn = parse_quote! {
        extern "C" fn jade_accumulate(ptr: u32, size: u32) -> (u64, u64) {
            #fun

            let buf = unsafe { core::slice::from_raw_parts(ptr as *const u8, size as usize) };
            let (slot, id, results): (u32, u32, u32) =
                jade::codec::decode(buf).expect("failed to decode accumulate parameters");
            if let Some(result) = #funame(slot, id, results) {
                ((&result).as_ptr() as u64, result.len() as u64)
            } else {
                (0, 0)
            }
        }
    };

    // export the function with polkavm-derive-impl
    match polkavm_derive_impl::polkavm_export(attrs, export) {
        Ok(stream) => stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
