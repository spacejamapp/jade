//! refine interface impl

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Implement the is_authorized interface
///
/// 1. wrap the function with a C-compatible function
/// 2. impl with polkavm-derive-impl
pub fn refine(_args: TokenStream, input: TokenStream) -> TokenStream {
    let fun = parse_macro_input!(input as syn::ItemFn);
    let funame = fun.sig.ident.clone();

    // construct the export
    //
    // TODO: introduce params check
    quote::quote! {
        #[jade::polkavm_derive::polkavm_export(abi = jade::polkavm_derive::default_abi)]
        extern "C" fn jade_refine(ptr: u32, size: u32) -> (u64, u64) {
            #fun

            let buf = unsafe { core::slice::from_raw_parts(ptr as *const u8, size as usize) };
            let (core, index, id, payload, package): (u16, u16, u32, Vec<u8>, OpaqueHash) =
                jade::codec::decode(buf).expect("failed to decode refine parameters");
            let result = #funame(core, index, id, payload, package);
            ((&result).as_ptr() as u64, result.len() as u64)
        }
    }
    .into()
}
