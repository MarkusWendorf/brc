RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data -Copt-level=3" cargo build --release

~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata merge -o ./merged.profdata /tmp/pgo-data

RUSTFLAGS="-Cprofile-use=/home/markus/code/brc/merged.profdata -O -Copt-level=3" cargo build --release