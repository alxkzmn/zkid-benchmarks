use criterion::{BatchSize, Criterion, criterion_group, criterion_main};

use gkr_engine::MPIConfig;
use sha256_expander_benchmark::bench::prepare;
use sha256_expander_benchmark::bench::prove;
use sha256_expander_benchmark::bench::verify;

fn criterion_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("sha256_expander");
    group.sample_size(10);

    let universe = MPIConfig::init().expect("Failed to initialize MPI");
    let world = universe.world();
    let mpi_config = MPIConfig::prover_new(Some(&universe), Some(&world));

    // Proving benchmark
    group.bench_function("sha256_expander_prove", |bench| {
        bench.iter_batched(
            || prepare(),
            |(circuit_file, witness_file)| {
                prove(&circuit_file, &witness_file, mpi_config.clone());
            },
            BatchSize::SmallInput,
        );
    });

    // Verify benchmark
    group.bench_function("sha256_expander_verify", |bench| {
        bench.iter_batched(
            || {
                // Prepare & prove to obtain proof for verification
                let (circuit_file, witness_file) = prepare();

                let (claimed_v, proof) = prove(&circuit_file, &witness_file, mpi_config.clone());

                (
                    circuit_file,
                    witness_file,
                    claimed_v,
                    proof,
                    mpi_config.clone(),
                )
            },
            |(circuit_file, witness_file, claimed, proof, mpi_config)| {
                // Set up verifier

                verify(&circuit_file, &witness_file, &proof, &claimed, mpi_config);
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(sha256_expander_benches, criterion_benchmarks);
criterion_main!(sha256_expander_benches);
