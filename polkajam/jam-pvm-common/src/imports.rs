#[polkavm_derive::polkavm_import]
extern "C" {
	// NOTE: This is NOT part of the GP.
	#[polkavm_import(index = 100)]
	pub fn log(
		level: u64,
		target_ptr: *const u8,
		target_len: u64,
		text_ptr: *const u8,
		text_len: u64,
	);

	#[polkavm_import(index = 0)]
	pub fn gas() -> u64;

	// If `service == u64::MAX`, then use caller service's storage.
	// Copies up to out_len bytes.
	// Returns `u64::MAX` if the preimage is unknown. Otherwise the preimage's length.
	#[polkavm_import(index = 1)]
	pub fn lookup(
		service: u64,
		hash_ptr: *const u8,
		out: *mut u8,
		offset: u64,
		out_len: u64,
	) -> u64;

	// If `service == u64::MAX`, then use caller service's storage.
	// Copies up to out_len bytes.
	// Returns `u64::MAX` if the key is non-existent. Otherwise the value's length.
	#[polkavm_import(index = 2)]
	pub fn read(
		service: u64,
		key_ptr: *const u8,
		key_len: u64,
		out: *mut u8,
		offset: u64,
		out_len: u64,
	) -> u64;

	// Returns the length of the *old* value or u64::MAX if there wasn't one.
	#[polkavm_import(index = 3)]
	pub fn write(key_ptr: *const u8, key_len: u64, value: *const u8, value_len: u64) -> u64;

	#[polkavm_import(index = 4)]
	pub fn info(service: u64, service_info_ptr: *mut u8) -> u64;

	#[polkavm_import(index = 5)]
	pub fn bless(m: u64, a: u64, v: u64, aa: *const u8, n: u64) -> u64;

	#[polkavm_import(index = 6)]
	pub fn assign(core: u64, auth_ptr: *const u8) -> u64;

	#[polkavm_import(index = 7)]
	pub fn designate(validator_keys_ptr: *const u8) -> u64;

	// Not available for on_transfer:
	#[polkavm_import(index = 8)]
	pub fn checkpoint();

	#[polkavm_import(index = 9)]
	pub fn new(
		code_hash_ptr: *const u8,
		code_len: u64,
		min_item_gas: u64,
		min_memo_gas: u64,
	) -> u64;

	#[polkavm_import(index = 10)]
	pub fn upgrade(code_hash_ptr: *const u8, min_item_gas: u64, min_memo_gas: u64) -> u64;

	#[polkavm_import(index = 11)]
	pub fn transfer(dest: u64, amount: u64, gas_limit: u64, memo_ptr: *const u8) -> u64;

	#[polkavm_import(index = 12)]
	pub fn eject(target: u64, code_hash: *const u8) -> u64;

	#[cfg_attr(not(target_arch = "riscv64"), allow(improper_ctypes))]
	#[polkavm_import(index = 13)]
	pub fn query(hash_ptr: *const u8, preimage_len: u64) -> (u64, u64);

	#[polkavm_import(index = 14)]
	pub fn solicit(hash_ptr: *const u8, preimage_len: u64) -> u64;

	#[polkavm_import(index = 15)]
	pub fn forget(hash_ptr: *const u8, preimage_len: u64) -> u64;

	#[polkavm_import(index = 16)]
	pub fn yield_hash(hash_ptr: *const u8) -> u64;

	#[polkavm_import(index = 27)]
	pub fn provide(service_id: u64, preimage_ptr: *const u8, preimage_len: u64) -> u64;

	#[polkavm_import(index = 17)]
	pub fn historical_lookup(
		service_id: u64,
		ho: *const u8,
		bo: *mut u8,
		offset: u64,
		bz: u64,
	) -> u64;

	#[polkavm_import(index = 18)]
	pub fn fetch(buffer: *mut u8, offset: u64, buffer_len: u64, kind: u64, a: u64, b: u64) -> u64;

	#[polkavm_import(index = 19)]
	pub fn export(buffer: *const u8, buffer_len: u64) -> u64;

	#[polkavm_import(index = 20)]
	pub fn machine(code_ptr: *const u8, code_len: u64, program_counter: u64) -> u64;

	#[polkavm_import(index = 21)]
	pub fn peek(vm_handle: u64, outer_dst: *mut u8, inner_src: u64, length: u64) -> u64;

	#[polkavm_import(index = 22)]
	pub fn poke(vm_handle: u64, outer_src: *const u8, inner_dst: u64, length: u64) -> u64;

	#[polkavm_import(index = 23)]
	pub fn zero(vm_handle: u64, page: u64, count: u64) -> u64;

	#[polkavm_import(index = 24)]
	pub fn void(vm_handle: u64, page: u64, count: u64) -> u64;

	// When this crate is compiled natively Rust will complain that the tuple
	// here cannot be used in FFI. This happens because for non-RISC-V targets
	// the `polkavm_import` macro just passes through the `extern C` block as-is.
	//
	// Compiling this natively is nonsense since calling anything would result
	// in a segfault anyway, so just silence the warning.
	#[cfg_attr(not(target_arch = "riscv64"), allow(improper_ctypes))]
	#[polkavm_import(index = 25)]
	pub fn invoke(vm_handle: u64, args: *mut core::ffi::c_void) -> (u64, u64);

	#[polkavm_import(index = 26)]
	pub fn expunge(vm_handle: u64) -> u64;
}
