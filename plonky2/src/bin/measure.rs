use plonky2::{plonk::config::PoseidonGoldilocksConfig, util::serialization::Write};
use plonky2_sha256::bench::{prove, sha256_no_lookup_prepare};
use plonky2_u32::gates::arithmetic_u32::{U32GateSerializer, U32GeneratorSerializer};

use utils::bench::measure_peak_memory;

const D: usize = 2;
type C = PoseidonGoldilocksConfig;

fn main() {
    let ((data, pw), peak_memory) = measure_peak_memory(|| sha256_no_lookup_prepare());

    println!(
        "Preprocessing peak memory: {} GB",
        peak_memory as f32 / (1024.0 * 1024.0 * 1024.0),
    );

    let gate_serializer = U32GateSerializer;
    let common_data_size = data.common.to_bytes(&gate_serializer).unwrap().len();
    let generator_serializer = U32GeneratorSerializer::<C, D>::default();
    let prover_data_size = data
        .prover_only
        .to_bytes(&generator_serializer, &data.common)
        .unwrap()
        .len();

    println!(
        "Common data size: {}B, Prover data size: {}B",
        common_data_size, prover_data_size
    );

    let (proof, peak_memory) = measure_peak_memory(|| prove(&data.prover_data(), pw));

    println!(
        "Proving peak memory: {} GB",
        peak_memory as f32 / (1024.0 * 1024.0 * 1024.0),
    );

    let mut buffer = Vec::new();
    buffer.write_proof(&proof.proof).unwrap();
    println!("Proof size: {} KB", buffer.len() as f32 / 1024.0);
}
