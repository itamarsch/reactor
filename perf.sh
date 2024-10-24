cargo build --release

perf record --call-graph dwarf target/release/reactor $1

perf script | ~/.cargo/bin/inferno-collapse-perf | ~/.cargo/bin/inferno-flamegraph >perf.svg

firefox perf.svg
