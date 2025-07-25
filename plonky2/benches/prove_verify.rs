use criterion::{BatchSize, Criterion, criterion_group, criterion_main};

use plonky2_sha256::bench::{prove, sha256_no_lookup_prepare, verify};

fn sha256_no_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("sha256_no_lookup");
    group.sample_size(10);

    group.bench_function("sha256_no_lookup_prove", |bench| {
        bench.iter_batched(
            || sha256_no_lookup_prepare(),
            |(data, pw)| {
                prove(&data.prover_data(), pw);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("sha256_no_lookup_verify", |bench| {
        bench.iter_batched(
            || {
                let (data, pw) = sha256_no_lookup_prepare();
                let verifier_data = data.verifier_data();
                (prove(&data.prover_data(), pw), verifier_data)
            },
            |(proof, data)| {
                verify(&data, proof);
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

criterion_main!(sha256);
criterion_group!(sha256, sha256_no_lookup);
