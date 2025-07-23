use jolt::Serializable;
use std::time::Instant;
use utils::{bench::benchmark, bench::Metrics, metadata::SHA2_INPUTS, sha2_input};

const TARGET_DIR: &str = "./sha2-guest";

fn main() {
    let csv_file = format!("sha2_jolt{}{}.csv", "", "");

    benchmark(benchmark_sha2, &SHA2_INPUTS, &csv_file);
}

fn benchmark_sha2(num_bytes: usize) -> Metrics {
    let mut metrics = Metrics::new(num_bytes as usize);

    let program = sha2_guest::compile_sha2(TARGET_DIR);
    let prover_preprocessing = sha2_guest::preprocess_prover_sha2(&program);
    let verifier_preprocessing = sha2_guest::preprocess_verifier_sha2(&program);

    let prover = sha2_guest::build_prover_sha2(program, prover_preprocessing);
    let verifier = sha2_guest::build_verifier_sha2(verifier_preprocessing);

    let input = sha2_input(num_bytes);
    let program_summary = sha2_guest::analyze_sha2(&input);
    metrics.cycles = program_summary.processed_trace.len() as u64;
    
    let start = Instant::now();
    let (output, proof) = prover(&input);
    metrics.proof_duration = start.elapsed();
    metrics.proof_size = proof.size().unwrap();

    let start = Instant::now();
    let _verify_result = verifier(&input, output, proof);
    metrics.verify_duration = start.elapsed();

    metrics
}
