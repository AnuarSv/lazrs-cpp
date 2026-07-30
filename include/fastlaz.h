#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

int32_t fastlaz_read_and_count(const char *filename, uint64_t *counts_out, uint64_t *total_out);

}  // extern "C"
