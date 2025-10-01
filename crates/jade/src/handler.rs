//! panic handler and allocator

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
#[global_allocator]
static ALLOCATOR: polkavm_derive::LeakingAllocator = polkavm_derive::LeakingAllocator;

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        core::arch::asm!("unimp", options(noreturn));
    }
}
