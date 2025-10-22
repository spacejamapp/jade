//! authorize interface impl

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Implement the is_authorized interface
///
/// 1. wrap the function with a C-compatible function
/// 2. impl with polkavm-derive-impl
pub fn accumulate(_args: TokenStream, input: TokenStream) -> TokenStream {
    let fun = parse_macro_input!(input as syn::ItemFn);
    let funame = fun.sig.ident.clone();

    // construct the export
    //
    // TODO: introduce params check
    quote::quote! {
        #[jade::polkavm_derive::polkavm_export(abi = jade::polkavm_derive::default_abi)]
        extern "C" fn jade_accumulate(ptr: u32, size: u32) -> (u64, u64) {
            #fun

            let buf = unsafe { core::slice::from_raw_parts(ptr as *const u8, size as usize) };
            let jade::service::vm::AccumulateParams {slot, id, results} =
                jade::codec::decode(buf).expect("failed to decode accumulate parameters");
            let items = jade::host::fetch::items().expect("failed to fetch accumulate items");
            if let Some(result) = #funame(slot, id, items) {
                ((&result).as_ptr() as u64, result.len() as u64)
            } else {
                (0, 0)
            }
        }
    }
    .into()
}
