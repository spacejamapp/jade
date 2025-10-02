//! authorize interface impl

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Implement the is_authorized interface
///
/// 1. wrap the function with a C-compatible function
/// 2. impl with polkavm-derive-impl
pub fn is_authorized(_args: TokenStream, input: TokenStream) -> TokenStream {
    let fun = parse_macro_input!(input as syn::ItemFn);
    let funame = fun.sig.ident.clone();

    // construct the export
    //
    // TODO: introduce params check
    quote::quote! {
        #[jade::polkavm_derive::polkavm_export(abi = jade::polkavm_derive::default_abi)]
        extern "C" fn jade_is_authorized(ptr: u32, size: u32) -> (u64, u64) {
            jade::info!("is_authorized");

            #fun

            let buf = unsafe { core::slice::from_raw_parts(ptr as *const u8, size as usize) };
            let core_index: CoreIndex =
                 jade::codec::decode(buf).inspect_err(|e| jade::error!("decoded is_authorized parameters: {:?}", e))
                     .expect("failed to decode is_authorized parameters");
            let result = #funame(core_index);
            ((&result).as_ptr() as u64, result.len() as u64)
        }
    }
    .into()
}
