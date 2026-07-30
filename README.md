# lazrs-cpp

A small C compatible library that exposes fast LAS/LAZ point cloud reading to C++, built on top of the Rust las crate (which itself wraps laz-rs for LAZ decompression).

## Why this exists

LAZ is the standard compressed format for LiDAR and mobile laser scanning point clouds, and reading it fast matters once files reach hundreds of millions of points. The common C++ options are PDAL, which is general purpose and carries real overhead for a task this narrow, or the older LASzip library. Benchmarking a 24.5 million point file showed that a naive PDAL based C++ reader took 6.65 seconds, while Python through laspy using the Rust based lazrs backend took 2.99 seconds. That gap is not about C++ versus Python as languages, it comes down to which underlying implementation is doing the decompression and how the data is pulled out of it afterwards.

This library calls into the Rust las crate directly from C++, using its bulk read path (parallel LAZ decompression via the laz-parallel feature) and its columnar field accessors (reading only the classification field straight out of the decoded buffer instead of decoding a full point struct per record). The result reads the same file in 1.82 seconds, faster than every other approach tested.

## Benchmark

All numbers are from the same 24.5 million point file, measured with isolated process runs and peak resident memory via getrusage.

| approach | time | peak memory |
|---|---|---|
| C++ with Rust las crate, naive point-by-point iteration | 11.36 s | - |
| Python with PDAL bindings | 10.09 s | 3308 MB |
| C++ with PDAL | 6.65 s | 1621 MB |
| Python with laspy and lazrs | 2.99 s | 1693 MB |
| C++ with Rust las crate, bulk read and columnar access | 1.82 s | 1086 MB |

The naive Rust attempt is included on purpose. Calling a fast library the slow way still gives slow results, the win came from using the bulk and columnar APIs rather than from Rust itself.

## Benchmark at larger scale

The numbers above are from a 24.5 million point file. To check whether the gap holds up on real, larger data, the same three approaches were measured again on a 1.6 GB file with 202.8 million points, using the same isolated process methodology.

| approach | time | peak memory |
|---|---|---|
| C++ with Rust las crate, bulk read and columnar access | 19.5 s | 8.6 GB |
| Python with laspy and lazrs | 40.7 s | 12.5 GB |
| C++ with PDAL | 111.6 s | 11.1 GB |

The gap does not shrink at this size, it widens. At 24.5 million points this library was 3.65x faster than the PDAL based reader and 1.64x faster than Python. At 202.8 million points that is 5.7x and 2.1x. Peak memory also stays well below both alternatives, which matters once files get large enough that memory becomes the actual limit on a given machine.

## What this project is and is not

The actual LAS parsing and LAZ decompression is done entirely by the las crate (github.com/gadomski/las-rs), which depends on laz-rs for the compression codec, both by Alexandre Montaigu, licensed under Apache 2.0. This repository does not reimplement any of that. What it adds is the C ABI surface (`fastlaz_read_and_count`) and the specific access pattern that makes the underlying library fast to use from a C or C++ codebase, along with a generated header via cbindgen and a small example program.

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
