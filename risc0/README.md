# RISC Zero(zkVM) SHA256 Benchmarks

The benchmark code is from the following one:

* https://github.com/kkrt-labs/zkvm-benchmarks/tree/master/risczero

## How to run

1. Install the RISC Zero tools (https://dev.risczero.com/api/zkvm/install)

2. Enter the dir

```console
cd risc0
```

3. Run the bench & measure the RAM usage

```console
RUSTFLAGS="-C target-cpu=native" cargo run --release --bin sha2
```
