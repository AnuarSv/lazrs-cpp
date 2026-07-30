use std::ffi::CStr;
use std::os::raw::c_char;

/// Reads a LAS/LAZ file and counts points per classification value.
///
/// `counts_out` must point to an already-allocated array of 256 u64 values.
/// Returns 0 on success, -1 on error.
#[unsafe(no_mangle)]
pub extern "C" fn fastlaz_read_and_count(
    filename: *const c_char,
    counts_out: *mut u64,
    total_out: *mut u64,
) -> i32 {
    let filename = unsafe {
        match CStr::from_ptr(filename).to_str() {
            Ok(s) => s,
            Err(_) => return -1,
        }
    };

    let mut reader = match las::Reader::from_path(filename) {
        Ok(r) => r,
        Err(_) => return -1,
    };

    let point_data = match reader.read_all() {
        Ok(pd) => pd,
        Err(_) => return -1,
    };

    let counts = unsafe { std::slice::from_raw_parts_mut(counts_out, 256) };
    let mut total: u64 = 0;

    for cls in point_data.classification() {
        counts[cls as usize] += 1;
        total += 1;
    }

    unsafe {
        *total_out = total;
    }

    0
}
