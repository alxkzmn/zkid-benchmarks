use noir_r1cs::NoirProofScheme;
use rand::RngCore;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::process::Command;
use std::time::Instant;
use utils::bench::{Metrics, benchmark};

pub const INPUT_EXP: [u32; 5] = [8, 9, 10, 11, 12];
pub const TMP_DIR: &str = "tmp";
pub const CIRCUIT_ROOT: &str = "circuits/hash/sha256-provekit";
pub const CSV_OUTPUT: &str = "tmp/provekit_sha256.csv";

/// Generates random input files for hashing benchmarks.
pub fn generate_hash_inputs() -> Result<(), &'static str> {
    let input_dir = format!("{}/hash-input", TMP_DIR);
    fs::create_dir_all(&input_dir).map_err(|_| "Failed to create input directory")?;

    let mut rng = rand::thread_rng();

    for exp in INPUT_EXP {
        let size = 1usize << exp;
        let bin_path = format!("{}/input_2e{}.bin", input_dir, exp);

        let mut data = vec![0u8; size];
        rng.fill_bytes(&mut data);

        let mut file = File::create(&bin_path).map_err(|_| "Failed to create input file")?;
        file.write_all(&data)
            .map_err(|_| "Failed to write input file")?;
    }

    Ok(())
}

/// Compiles all Noir circuits.
pub fn compile_all_circuits() -> Result<(), &'static str> {
    let output = Command::new("nargo")
        .args([
            "compile",
            "--silence-warnings",
            "--skip-brillig-constraints-check",
        ])
        .current_dir("circuits")
        .output()
        .map_err(|_| "Failed to execute nargo")?;

    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        return Err("Compilation failed");
    }

    Ok(())
}

/// Generates a Prover.toml file from input data.
pub fn generate_prover_toml(
    toml_path: &str,
    input_file: &str,
    size: usize,
) -> Result<(), &'static str> {
    let mut file = File::open(input_file).map_err(|_| "Failed to open input file")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|_| "Failed to read input file")?;

    let content = format!(
        "input = [\n{}\n]\ninput_len = {}\n",
        buffer
            .iter()
            .map(|b| format!("    {}", b))
            .collect::<Vec<_>>()
            .join(",\n"),
        size
    );

    fs::write(toml_path, content).map_err(|_| "Failed to write Prover.toml")?;

    Ok(())
}

/// Sets up Prover.toml files for all circuits.
pub fn setup_circuits() -> Result<(), &'static str> {
    for exp in INPUT_EXP {
        let size = 1usize << exp;
        let input_file = format!("{}/hash-input/input_2e{}.bin", TMP_DIR, exp);
        let circuit_dir = format!("{}/sha256-bench-2e{}", CIRCUIT_ROOT, exp);
        let toml_path = format!("{}/Prover.toml", circuit_dir);

        generate_prover_toml(&toml_path, &input_file, size)?;
    }

    Ok(())
}

/// Sets up the benchmark.
pub fn setup() -> Result<(), &'static str> {
    fs::create_dir_all(TMP_DIR).map_err(|_| "Failed to create tmp directory")?;

    generate_hash_inputs()?;
    compile_all_circuits()?;
    setup_circuits()?;

    Ok(())
}

/// Benchmarks provekit with sha256 for all input exponents.
pub fn bench_sha256() {
    let inputs: Vec<u32> = INPUT_EXP.to_vec();

    benchmark(
        |input_exp| {
            let size = 1usize << input_exp;
            let mut metrics = Metrics::new(size);

            let circuit_dir = format!("{}/sha256-bench-2e{}", CIRCUIT_ROOT, input_exp);
            let circuit_path = format!("circuits/target/sha256_bench_2e{}.json", input_exp);
            let prover_toml_path = format!("{}/Prover.toml", circuit_dir);

            let proof_scheme = NoirProofScheme::from_file(&circuit_path).unwrap();
            let input_map = proof_scheme.read_witness(&prover_toml_path).unwrap();

            let prove_start = Instant::now();
            let proof = proof_scheme.prove(&input_map).unwrap();
            metrics.proof_duration = prove_start.elapsed();

            let verify_start = Instant::now();
            proof_scheme.verify(&proof).unwrap();
            metrics.verify_duration = verify_start.elapsed();

            metrics.proof_size = proof.whir_r1cs_proof.transcript.len();

            metrics
        },
        &inputs,
        CSV_OUTPUT,
    );
}
