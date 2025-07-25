# Plonky3 SP1 SHA256 Benchmark

This code is based on https://github.com/babybear-labs/benchmark/tree/main/sp1/algos/sha256.

## Requirements

- [Rust](https://rustup.rs/)
- [SP1](https://docs.succinct.xyz/getting-started/install.html)

## RAM Usage Measurement for SHA256

1. Build the project (generates the sha binary in ../target/release):

```
cd script
cargo build --release
```

2. Run a "no-measure" setup to generate the proving key (pk.bin), verifying key (vk.bin), and proof (proof.bin):

```
cd script
# Generate and write pk.bin and vk.bin:
cargo run --release -- --setup
# Generate and write proof:
cargo run --release -- --prove
```

3. Measure the RAM usage:

```
cd script
cargo run --bin measure --release 
```

## Benchmarking

```
cd script
cargo bench
```
