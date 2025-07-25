use sp1_sdk::{ProverClient, SP1Stdin, include_elf};
use utils::bench::measure_peak_memory;

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const SHA_ELF: &[u8] = include_elf!("sha-program");

fn main() {
    // Setup the prover client.
    let client = ProverClient::from_env();
    let stdin = SP1Stdin::new();

    // Setup the program for proving.
    let ((_, _), peak_memory) = measure_peak_memory(|| client.setup(SHA_ELF));

    println!(
        "Preprocessing peak memory: {} GB",
        peak_memory as f32 / (1024.0 * 1024.0 * 1024.0)
    );

    // Load the proving key and verifying key from the files.
    let pk_bytes = std::fs::read("pk.bin").expect("Unable to read file");
    let pk: sp1_sdk::SP1ProvingKey = bincode::deserialize(&pk_bytes).unwrap();

    // Generate the proof
    let (_, peak_memory) = measure_peak_memory(|| {
        client
            .prove(&pk, &stdin)
            .run()
            .expect("failed to generate proof")
    });

    println!(
        "Proving peak memory: {} GB",
        peak_memory as f32 / (1024.0 * 1024.0 * 1024.0),
    );
}
