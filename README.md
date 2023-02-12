# librusthelloworld
C shared lib done using Rust


## Build

With debug info:

    cargo build

Release build (way faster):

    cargo build --release

Generating C header files:

    cargo test --features c-headers -- generate_headers --nocapture
