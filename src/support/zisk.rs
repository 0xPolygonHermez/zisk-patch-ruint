extern "C" {
    pub fn mul256(result: *mut u64, x: *const u64, y: *const u64);
    pub fn div256(result: *mut u64, x: *const u64, y: *const u64);
    pub fn rem256(result: *mut u64, x: *const u64, y: *const u64);
}