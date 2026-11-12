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

/// Returns the number of points in a LAS/LAZ file by reading only the header.
/// Returns -1 on error.
#[unsafe(no_mangle)]
pub extern "C" fn fastlaz_point_count(filename: *const c_char) -> i64 {
    let filename = unsafe {
        match CStr::from_ptr(filename).to_str() {
            Ok(s) => s,
            Err(_) => return -1,
        }
    };

    let reader = match las::Reader::from_path(filename) {
        Ok(r) => r,
        Err(_) => return -1,
    };

    reader.header().number_of_points() as i64
}

/// Reads X, Y, Z coordinates from a LAS/LAZ file into caller-allocated buffers.
///
/// `x_out`, `y_out`, `z_out` must each point to an already-allocated array of
/// `capacity` f64 values (get `capacity` from `fastlaz_point_count` first).
/// Returns the number of points written, or -1 on error, or -2 if `capacity`
/// is smaller than the file's point count.
#[unsafe(no_mangle)]
pub extern "C" fn fastlaz_read_xyz(
    filename: *const c_char,
    x_out: *mut f64,
    y_out: *mut f64,
    z_out: *mut f64,
    capacity: u64,
) -> i64 {
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

    if reader.header().number_of_points() > capacity {
        return -2;
    }

    let x = unsafe { std::slice::from_raw_parts_mut(x_out, capacity as usize) };
    let y = unsafe { std::slice::from_raw_parts_mut(y_out, capacity as usize) };
    let z = unsafe { std::slice::from_raw_parts_mut(z_out, capacity as usize) };

    // Read in bounded chunks so the raw decompressed buffer for one chunk
    // is freed before the next chunk is decompressed, instead of holding
    // the whole file's raw point records and the x/y/z arrays at once.
    const CHUNK: u64 = 5_000_000;
    let mut point_data = las::PointDataBuilder::new()
        .for_header(reader.header())
        .build();
    let mut total: i64 = 0;

    loop {
        let filled = match reader.fill_points(CHUNK, &mut point_data) {
            Ok(n) => n,
            Err(_) => return -1,
        };
        if filled == 0 {
            break;
        }

        let offset = total as usize;
        for (i, ((xv, yv), zv)) in point_data
            .x()
            .zip(point_data.y())
            .zip(point_data.z())
            .enumerate()
        {
            x[offset + i] = xv;
            y[offset + i] = yv;
            z[offset + i] = zv;
        }
        total += filled as i64;
    }

    total
}

/// Reads R, G, B color into caller-allocated buffers.
///
/// `r_out`, `g_out`, `b_out` must each point to an already-allocated array of
/// `capacity` u16 values (get `capacity` from `fastlaz_point_count` first).
/// Returns the number of points written, or -1 on error, -2 if `capacity` is
/// smaller than the file's point count, or -3 if the file has no color data.
#[unsafe(no_mangle)]
pub extern "C" fn fastlaz_read_rgb(
    filename: *const c_char,
    r_out: *mut u16,
    g_out: *mut u16,
    b_out: *mut u16,
    capacity: u64,
) -> i64 {
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

    if reader.header().number_of_points() > capacity {
        return -2;
    }

    let r = unsafe { std::slice::from_raw_parts_mut(r_out, capacity as usize) };
    let g = unsafe { std::slice::from_raw_parts_mut(g_out, capacity as usize) };
    let b = unsafe { std::slice::from_raw_parts_mut(b_out, capacity as usize) };

    const CHUNK: u64 = 5_000_000;
    let mut point_data = las::PointDataBuilder::new()
        .for_header(reader.header())
        .build();
    let mut total: i64 = 0;

    loop {
        let filled = match reader.fill_points(CHUNK, &mut point_data) {
            Ok(n) => n,
            Err(_) => return -1,
        };
        if filled == 0 {
            break;
        }

        let rgb = match point_data.rgb() {
            Some(iter) => iter,
            None => return -3,
        };

        let offset = total as usize;
        for (i, (rv, gv, bv)) in rgb.enumerate() {
            r[offset + i] = rv;
            g[offset + i] = gv;
            b[offset + i] = bv;
        }
        total += filled as i64;
    }

    total
}
