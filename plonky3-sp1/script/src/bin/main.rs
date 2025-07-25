//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use clap::Parser;
use sp1_sdk::{ProverClient, SP1Stdin, include_elf};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const SHA_ELF: &[u8] = include_elf!("sha-program");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    setup: bool,

    #[clap(long)]
    setup_no_write: bool,

    #[clap(long)]
    prove: bool,

    #[clap(long)]
    prove_no_write: bool,

    #[clap(long)]
    verify: bool,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = Args::parse();

    if !args.execute
        && !args.setup
        && !args.setup_no_write
        && !args.prove_no_write
        && !args.prove
        && !args.verify
    {
        eprintln!(
            "Error: You must specify either --execute, --setup, --setup_no_write, --prove, --prove_no_write or --verify"
        );
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::from_env();

    let stdin = SP1Stdin::new();

    if args.execute {
        // Execute the program
        let (_output, report) = client.execute(SHA_ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");

        println!("Number of cycles: {}", report.total_instruction_count());
    } else if args.setup {
        // Setup the program for proving.
        let (pk, vk) = client.setup(SHA_ELF);

        println!("ELF size: {} KB", pk.elf.len() as f32 / 1024.0);
        let pk_bytes = bincode::serialize(&pk).unwrap();
        println!(
            "Proving key size: {} MB",
            pk_bytes.len() as f32 / (1024.0 * 1024.0)
        );
        std::fs::write("pk.bin", pk_bytes).expect("Unable to write file");
        let vk_bytes = bincode::serialize(&vk).unwrap();
        println!(
            "Verifying key size: {} MB",
            vk_bytes.len() as f32 / (1024.0 * 1024.0)
        );
        std::fs::write("vk.bin", vk_bytes).expect("Unable to write file");
    } else if args.setup_no_write {
        // Setup the program for proving.
        let (_, _) = client.setup(SHA_ELF);
        println!("Setup completed successfully.");
        std::thread::sleep(std::time::Duration::from_millis(200));
    } else if args.prove {
        // Load the proving key and verifying key from the files.
        let pk_bytes = std::fs::read("pk.bin").expect("Unable to read file");
        let pk: sp1_sdk::SP1ProvingKey = bincode::deserialize(&pk_bytes).unwrap();
        // Generate the proof
        let proof = client
            .prove(&pk, &stdin)
            .run()
            .expect("failed to generate proof");
        println!("Successfully generated proof!");

        let proof_bytes = bincode::serialize(&proof).unwrap();
        println!(
            "Proof size: {} MB",
            proof_bytes.len() as f32 / (1024.0 * 1024.0)
        );
        std::fs::write("proof.bin", proof_bytes).expect("Unable to write file");
        println!("Proof written to proof.bin");
    } else if args.prove_no_write {
        // Load the proving key and verifying key from the files.
        let pk_bytes = std::fs::read("pk.bin").expect("Unable to read file");
        let pk: sp1_sdk::SP1ProvingKey = bincode::deserialize(&pk_bytes).unwrap();
        // Generate the proof
        let _ = client
            .prove(&pk, &stdin)
            .run()
            .expect("failed to generate proof");
        println!("Successfully generated proof!");
        std::thread::sleep(std::time::Duration::from_millis(200));
    } else if args.verify {
        // Load the verifying key from the file.
        let vk_bytes = std::fs::read("vk.bin").expect("Unable to read file");
        let vk: sp1_sdk::SP1VerifyingKey = bincode::deserialize(&vk_bytes).unwrap();
        // Load the proof from the file.
        let proof_bytes = std::fs::read("proof.bin").expect("Unable to read file");
        let proof: sp1_sdk::SP1ProofWithPublicValues = bincode::deserialize(&proof_bytes).unwrap();

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
