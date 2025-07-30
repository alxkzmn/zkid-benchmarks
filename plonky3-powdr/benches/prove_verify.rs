use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use sha::bench::{prepare_pipeline, prove, verify};

fn sha256_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("sha256_bench");
    group.sample_size(10);

    group.bench_function("sha256_bench_prove", |bench| {
        bench.iter_batched(
            prepare_pipeline,
            |mut pipeline| {
                prove(&mut pipeline);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("sha256_bench_verify", |bench| {
        bench.iter_batched(
            || {
                let mut pipeline = prepare_pipeline();
                prove(&mut pipeline);
                pipeline
            },
            |pipeline| {
                verify(pipeline);
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

criterion_main!(sha256);
criterion_group!(sha256, sha256_bench);
