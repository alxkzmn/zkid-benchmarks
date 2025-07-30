# Plonky2 SHA256 Benchmark

This code is a fork of https://github.com/polymerdao/plonky2-sha256/tree/main

> [!NOTE]  
> The custom dependency for `plonky2_u32` is necessary because the original outdated `plonky2_u32` depended on an old version of `plonky2` and did not support serialization, so it would be impossible to measure PK/VK/proof size. Awaiting PR merge (https://github.com/0xPolygonZero/plonky2-u32/pull/7)

## Benchmarking SHA256 with Plonky2

```
cargo bench
```

## Measuring disk space and RAM requirements

```
cargo run --bin plonky2-measure --release
```
