# Changelog

## 0.2.2

Added CI (GitHub Actions build check), badges, and this changelog. No
code changes.

## 0.2.1

Corrected the benchmark methodology in the README. The classification
benchmark previously compared `fastlaz_read_and_count` against a Python
script that also loaded X, Y, Z, an unequal amount of work. Re-measured
with matched workloads: the classification gap is much smaller than
previously claimed, and Python edges ahead at 202.8 million points. No
code changes.

## 0.2.0

Added `fastlaz_point_count` and `fastlaz_read_xyz` for reading point
coordinates, not just classification counts. Point data is read in
bounded chunks instead of all at once, which cut peak memory on a
202.8 million point file from 12.4 GB to 6.4 GB and, by avoiding swap,
cut load time from 41s to 17s.

## 0.1.1

Added a benchmark at larger scale (202.8 million points) to the README.
No code changes.

## 0.1.0

Initial release. `fastlaz_read_and_count` reads a LAS/LAZ file and
counts points per classification value, using the `las` crate's bulk
read path and columnar field access.
