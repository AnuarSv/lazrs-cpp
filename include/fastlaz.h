#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

/// Reads a LAS/LAZ file and counts points per classification value.
///
/// `counts_out` must point to an already-allocated array of 256 u64 values.
/// Returns 0 on success, -1 on error.
int32_t fastlaz_read_and_count(const char *filename, uint64_t *counts_out, uint64_t *total_out);

/// Returns the number of points in a LAS/LAZ file by reading only the header.
/// Returns -1 on error.
int64_t fastlaz_point_count(const char *filename);

/// Reads X, Y, Z coordinates from a LAS/LAZ file into caller-allocated buffers.
///
/// `x_out`, `y_out`, `z_out` must each point to an already-allocated array of
/// `capacity` f64 values (get `capacity` from `fastlaz_point_count` first).
/// Returns the number of points written, or -1 on error, or -2 if `capacity`
/// is smaller than the file's point count.
int64_t fastlaz_read_xyz(const char *filename,
                         double *x_out,
                         double *y_out,
                         double *z_out,
                         uint64_t capacity);

/// Reads R, G, B color into caller-allocated buffers.
///
/// `r_out`, `g_out`, `b_out` must each point to an already-allocated array of
/// `capacity` u16 values (get `capacity` from `fastlaz_point_count` first).
/// Returns the number of points written, or -1 on error, -2 if `capacity` is
/// smaller than the file's point count, or -3 if the file has no color data.
int64_t fastlaz_read_rgb(const char *filename,
                         uint16_t *r_out,
                         uint16_t *g_out,
                         uint16_t *b_out,
                         uint64_t capacity);

}  // extern "C"
