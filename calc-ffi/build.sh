#!/bin/bash

set -euxo pipefail

cargo build
cbindgen -d -l c > ./calc_ffi.h

gcc \
    -I. \
    ./demo.c \
    -L./target/debug \
    -l:libcalc_ffi.a \
    -lpthread \
    -ldl \
    -o demo
