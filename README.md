# lazrs-cpp

A small C compatible library that exposes fast LAS/LAZ point cloud reading to C++, built on top of the Rust las crate (which itself wraps laz-rs for LAZ decompression).

## Why this exists

LAZ is the standard compressed format for LiDAR and mobile laser scanning point clouds, and reading it fast matters once files reach hundreds of millions of points. The common C++ option is PDAL, which is general purpose and carries real overhead for a task this narrow: reading a 202.8 million point file and extracting X, Y, Z takes PDAL 151 seconds and 13.4 GB of peak memory in the benchmarks below, against 15.35 seconds and 4.9 GB for this library. That gap is not about C++ versus other languages, it comes down to which underlying implementation is doing the decompression, how the data is pulled out of it afterwards, and how carefully memory is managed while that happens.

This library calls into the Rust las crate directly from C++, using its bulk read path (parallel LAZ decompression via the laz-parallel feature), its columnar field accessors (reading a field straight out of the decoded buffer instead of decoding a full point struct per record), and chunked reads that keep peak memory well below what a single bulk read needs. The numbers below are measured, not assumed, including the cases where the advantage turned out to be smaller than expected or nonexistent.

## Benchmark

All numbers are isolated process runs (peak resident memory via getrusage), on the same two files: a 24.5 million point file and a 202.8 million point (1.6 GB) file. Every approach measured does the same amount of work as `fastlaz`: it only reads what that function reads, nothing extra. The fastest result in each table is in **bold**.

### Reading classification

`fastlaz_read_and_count` decompresses a file and counts points per classification value.

| approach | 24.5M points | 202.8M points |
|---|---|---|
| **fastlaz** | **1.82 s / 1086 MB** | 19.5 s / 8.6 GB |
| Python, laspy + lazrs | 2.32 s / 1124 MB | **17.6 s / 8.7 GB** |
| C++, naive Rust las crate (point-by-point) | 11.36 s / - | not tested |
| Python, PDAL bindings | 10.09 s / 3308 MB | not tested |
| C++, PDAL | 6.65 s / 1621 MB | 111.6 s / 11.1 GB |

This one is close, and at the larger size Python edges ahead. Both `fastlaz` and laspy's lazrs backend decompress the exact same way under the hood (the LAZ codec doesn't skip fields it isn't asked for, the whole point record gets decompressed regardless), so this benchmark is really measuring small differences in decompression parallelism and per-point extraction overhead, not a fundamental advantage. Included for completeness rather than as a selling point.

### Reading X, Y, Z coordinates

`fastlaz_read_xyz` decompresses a file and fills X, Y, Z arrays. This is the more useful operation in practice, and where the library's actual advantage shows up.

| approach | 24.5M points | 202.8M points |
|---|---|---|
| **fastlaz** | **1.91 s / 832 MB** | **15.35 s / 4.9 GB** |
| C++, native Rust las crate, no chunking | 2.09 s / 1648 MB | 17.12 s / 13.3 GB |
| Python, laspy + lazrs | 2.39 s / 1668 MB | 19.31 s / 13.2 GB |
| C++, PDAL | 7.48 s / 2386 MB | 151.15 s / 13.4 GB |

The time advantage over Python and plain Rust is modest, 10 to 25 percent. The memory advantage is not: at 202.8 million points `fastlaz` uses roughly a third of the memory every other approach needs, including plain Rust with no chunking. That gap is explained below. Against PDAL specifically, both time and memory diverge sharply at scale.

## How it stays memory efficient

Point data is read in bounded chunks (5 million points at a time) rather than all at once. Reading the whole file into one buffer and then copying it into separate X, Y, Z arrays means holding both the raw buffer and the arrays in memory at the same time, which roughly doubles peak memory for no benefit, and that doubling is exactly what the "native Rust, no chunking" row above shows costing 13.3 GB instead of 4.9 GB. Chunked reading keeps only one chunk's raw buffer alive at a time. On the 202.8 million point file, switching to chunked reads cut peak memory from 12.4 GB to 6.4 GB in an earlier version of this benchmark, and cut load time from 41 seconds to 17 seconds as a side effect, because the process was no longer swapping.

## What this project is and is not

The actual LAS parsing and LAZ decompression is done entirely by the las crate (github.com/gadomski/las-rs), which depends on laz-rs for the compression codec, both by Alexandre Montaigu, licensed under Apache 2.0. This repository does not reimplement any of that. What it adds is the C ABI surface (`fastlaz_read_and_count`, `fastlaz_point_count`, `fastlaz_read_xyz`) and the specific access patterns that make the underlying library fast and memory efficient to use from a C or C++ codebase, along with a generated header via cbindgen and example programs.

## Building

Requires a Rust toolchain and a C++ compiler.

cargo build --release

This produces target/release/libfastlaz.so and a header at include/fastlaz.h (regenerate it with cbindgen if the Rust API changes).

To build and run the example:

cd example
g++ -std=c++17 -O2 example.cpp -o example_bin -I../include -I../target/release -L../target/release -lfastlaz -Wl,-rpath,'$ORIGIN/../target/release'
./example_bin path/to/file.laz

## License

Apache 2.0, see LICENSE. This matches the license of las-rs and laz-rs, which this project depends on and builds directly on top of.
