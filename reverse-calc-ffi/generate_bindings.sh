#!/bin/bash

set -euxo pipefail

# Remove exisiting file if any, and emit some linter settings
# to remove some noise at compile time
echo "#![allow(dead_code, improper_ctypes, non_camel_case_types, non_upper_case_globals)]" > ./src/bindings.rs
echo "" >> ./src/bindings.rs

# Generate the files, appending to the existing file
bindgen ../calc-ffi/calc_ffi.h >> src/bindings.rs
