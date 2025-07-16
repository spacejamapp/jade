use codec::{Decode, Encode, Output};
pub use core::alloc::Layout;

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

pub fn alloc(size: u32) -> u32 {
	let ptr = unsafe { alloc::alloc::alloc(Layout::from_size_align(size as usize, 4).unwrap()) };
	ptr as u32
}

pub fn dealloc(ptr: u32, size: u32) {
	unsafe {
		alloc::alloc::dealloc(ptr as *mut u8, Layout::from_size_align(size as usize, 4).unwrap())
	};
}

pub struct BufferOutput<'a>(&'a mut [u8], usize);
impl Output for BufferOutput<'_> {
	/// Write to the output.
	fn write(&mut self, bytes: &[u8]) {
		let (_, rest) = self.0.split_at_mut(self.1);
		let len = bytes.len().min(rest.len());
		rest[..len].copy_from_slice(&bytes[..len]);
		self.1 += len;
	}
}

pub fn decode_buf<T: Decode>(ptr: u32, size: u32) -> T {
	let slice = unsafe { core::slice::from_raw_parts(ptr as *const u8, size as usize) };
	let params = T::decode(&mut &slice[..]);
	dealloc(ptr, size);
	params.unwrap()
}

pub fn encode_to_buf<T: Encode>(value: T) -> (u32, u32) {
	// TODO: @gav wish avoid extra copy
	let size = value.encoded_size();
	let ptr = alloc(size as u32);
	let slice = unsafe { core::slice::from_raw_parts_mut(ptr as *mut u8, size) };
	value.encode_to(&mut BufferOutput(&mut slice[..], 0));
	(ptr, size as u32)
}
