#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub use {codec, jade_derive::*, polkavm_derive, service};

pub mod host;
pub mod logging;
pub mod prelude;

#[cfg(not(target_arch = "riscv64"))]
pub use {cjam, testing};

#[cfg(target_arch = "riscv64")]
#[global_allocator]
static ALLOCATOR: polkavm_derive::LeakingAllocator = polkavm_derive::LeakingAllocator;

#[cfg(target_arch = "riscv64")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        core::arch::asm!("unimp", options(noreturn));
    }
}
