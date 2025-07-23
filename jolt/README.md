# Jolt (zkVM) SHA256 Benchmarks

The benchmark code is from the following one:

* https://github.com/kkrt-labs/zkvm-benchmarks/tree/master/jolt

## How to run

1. Enter the dir

```console
cd jolt
```

2. Run the benchmark

```console
RUSTFLAGS="-C target-cpu=native" cargo run --release --bin sha2
```
