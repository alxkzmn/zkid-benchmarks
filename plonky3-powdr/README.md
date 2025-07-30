# Plonky3 Powdr SHA256 Benchmark

This benchmark code is based on https://github.com/babybear-labs/benchmark/tree/main/powdr/sha.

## Usage

This will run the host and generate ZK proofs.

```bash
cargo run -r --bin plonky3-powdr-sha
```

## Benchmarking

```bash
cargo bench
```

Measure RAM footprint:

```bash
cargo run -r --bin plonky3-powdr-measure
```
