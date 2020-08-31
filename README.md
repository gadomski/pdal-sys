# pdal-sys

Rust [*-sys package](https://doc.rust-lang.org/cargo/reference/build-scripts.html#-sys-packages) for [PDAL](https://pdal.io/) via [PDAL-C](https://github.com/PDAL/CAPI).
PDAL is a point cloud processing software library.

## Setting search paths

If **PDAL-C** is not installed to a default-searched prefix, you might have to set the following environment variables to build/test with cargo:

- `export BINDGEN_EXTRA_CLANG_ARGS="-I'/path/to/pdal-c/build/include'"`
- `export LIBRARY_PATH=/path/to/pdal-c/build/lib`