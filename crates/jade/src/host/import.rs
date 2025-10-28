//! Imports of host calls

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

    /// Get the gas used
    #[polkavm_import(index = 0)]
    pub fn gas() -> u64;

    /// Fetch a value from the storage
    #[polkavm_import(index = 1)]
    pub fn fetch(buffer: *mut u8, offset: u64, buffer_len: u64, kind: u64, a: u64, b: u64) -> u64;

    /// Retrieve a preimage by hash.
    #[polkavm_import(index = 2)]
    pub fn lookup(
        service: u64,
        hash_ptr: *const u8,
        out: *mut u8,
        offset: u64,
        out_len: u64,
    ) -> u64;

    /// Read a value from the storage
    #[polkavm_import(index = 3)]
    pub fn read(
        service: u64,
        key_ptr: *const u8,
        key_len: u64,
        out: *mut u8,
        offset: u64,
        out_len: u64,
    ) -> u64;

    /// Write a value to the storage
    #[polkavm_import(index = 4)]
    pub fn write(key_ptr: *const u8, key_len: u64, value: *const u8, value_len: u64) -> u64;

    /// Get the info of the service
    #[polkavm_import(index = 5)]
    pub fn info(service: u64, service_info_ptr: *mut u8) -> u64;
}
