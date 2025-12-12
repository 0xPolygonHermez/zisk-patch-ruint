
unsafe extern "C" {
    pub fn redmod256_c(a: *const u64, m: *const u64, result: *mut u64);
    pub fn addmod256_c(a: *const u64, b: *const u64, m: *const u64, result: *mut u64);
    pub fn mulmod256_c(a: *const u64, b: *const u64, m: *const u64, result: *mut u64);
    pub fn wmul256_c(a: *const u64, b: *const u64, result: *mut u64);
    pub fn omul256_c(a: *const u64, b: *const u64, result: *mut u64) -> bool;
    pub fn divrem256_c(a: *const u64, b: *const u64, q: *mut u64, r: *mut u64);
    pub fn wpow256_c(a: *const u64, exp: *const u64, result: *mut u64);
}