use criterion::{BatchSize, Criterion, criterion_group, criterion_main};

use sp1_sdk::{EnvProver, ProverClient, SP1ProvingKey, SP1Stdin, SP1VerifyingKey, include_elf};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const SHA_ELF: &[u8] = include_elf!("sha-program");

fn sha256_prepare(client: &EnvProver) -> (SP1ProvingKey, SP1VerifyingKey) {
    client.setup(SHA_ELF)
}

fn sha256_bench(c: &mut Criterion) {
    let client = ProverClient::from_env();
    let stdin = SP1Stdin::new();
    // // Setup the program for proving.
    // let (pk, vk) = client.setup(SHA_ELF);

    // // Generate the proof
    // let proof = client
    //     .prove(&pk, &stdin)
    //     .run()
    //     .expect("failed to generate proof");

    // println!("Successfully generated proof!");

    // // Verify the proof.
    // client.verify(&proof, &vk).expect("failed to verify proof");
    // println!("Successfully verified proof!");

    let mut group = c.benchmark_group("sha256_bench");
    group.sample_size(10);

    group.bench_function("sha256_bench_prove", |bench| {
        bench.iter_batched(
            || sha256_prepare(&client),
            |(pk, _pw)| {
                client
                    .prove(&pk, &stdin)
                    .run()
                    .expect("failed to generate proof");
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("sha256_bench_verify", |bench| {
        bench.iter_batched(
            || {
                let (pk, vk) = sha256_prepare(&client);
                (
                    client
                        .prove(&pk, &stdin)
                        .run()
                        .expect("failed to generate proof"),
                    vk,
                )
            },
            |(proof, vk)| {
                client.verify(&proof, &vk).expect("failed to verify proof");
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

criterion_main!(sha256);
criterion_group!(sha256, sha256_bench);
