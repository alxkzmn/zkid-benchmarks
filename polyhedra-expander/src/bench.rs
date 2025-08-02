use ::circuit::Circuit;
use bin::executor::{prove as expander_prove, verify as expander_verify};
use circuit_std_rs::sha256::m31::sha256_var_bytes;
use expander_compiler::frontend::*;
use gkr::Verifier;
use gkr_engine::GKREngine;
use gkr_engine::GKRScheme;
use gkr_engine::M31x1Config;
use gkr_engine::MPIConfig;
use gkr_engine::MPIEngine;
use gkr_engine::MPISharedMemory;
use gkr_engine::Proof;
use gkr_hashers::SHA256hasher;
use mersenne31::M31;
use mersenne31::M31Ext3;
use poly_commit::OrionPCSForGKR;
use rand::RngCore;
use serdes::ExpSerde;
use sha2::{Digest, Sha256};
use transcript::BytesHashTranscript;

// Constants and circuit definition
const INPUT_LEN: usize = 2048;
const OUTPUT_LEN: usize = 32; // SHA-256 digest length

declare_circuit!(SHA256Circuit {
    input: [Variable; INPUT_LEN],
    output: [Variable; OUTPUT_LEN],
});

// M31SingleConfig is a configuration for the M31 without SIMD (i.e. no parallel proving in Expander).
#[derive(Default, Debug, Clone, PartialOrd, Ord, Hash, PartialEq, Eq, Copy)]
pub struct M31SnglConfig<'a> {
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> GKREngine for M31SnglConfig<'a> {
    type FieldConfig = M31x1Config;
    type MPIConfig = MPIConfig<'a>;
    type TranscriptConfig = BytesHashTranscript<SHA256hasher>;
    type PCSConfig = OrionPCSForGKR<M31x1Config, M31>;
    const SCHEME: GKRScheme = GKRScheme::Vanilla;
}

pub type M31SingleConfig = M31SnglConfig<'static>;

impl Config for M31SingleConfig {
    const CONFIG_ID: usize = 1;
}

// Define the circuit
impl Define<M31SingleConfig> for SHA256Circuit<Variable> {
    fn define<Builder: RootAPI<M31SingleConfig>>(&self, api: &mut Builder) {
        let mut data = self.input.to_vec();
        data.extend(self.output.to_vec());
        api.memorized_simple_call(|api, data| check_sha256_var(api, data), &data);
    }
}

// Inspired by https://github.com/PolyhedraZK/ExpanderCompilerCollection/blob/cc12748abcf043c5912b36d3341e5be51b1dca61/circuit-std-rs/src/sha256/m31.rs#L515
pub fn check_sha256_var<C: Config, B: RootAPI<C>>(
    api: &mut B,
    data: &[Variable], //  msg ‖ digest
) -> Vec<Variable> {
    let msg_len = data.len() - OUTPUT_LEN;
    let expected = data[msg_len..].to_vec();
    let computed = sha256_var_bytes(api, &data[..msg_len]);

    for i in 0..OUTPUT_LEN {
        api.assert_is_equal(computed[i], expected[i]);
    }
    computed
}

/// Prepare a fresh circuit / witness pair
pub fn prepare() -> (String, String) {
    // Compile circuit
    let compile_result = compile(&SHA256Circuit::default(), CompileOptions::default()).unwrap();

    // Generate random message and compute its SHA-256
    let mut rng = rand::rng();
    let data = [rng.next_u32() as u8; INPUT_LEN];
    let mut hash = Sha256::new();
    hash.update(data);
    let output = hash.finalize();

    // Fill assignment
    let mut assignment = SHA256Circuit::default();
    for (i, input_byte) in data.iter().enumerate().take(INPUT_LEN) {
        assignment.input[i] = M31::from(*input_byte as u32);
    }
    for i in 0..OUTPUT_LEN {
        assignment.output[i] = M31::from(output[i] as u32);
    }

    let witness = compile_result
        .witness_solver
        .solve_witness_with_hints(&assignment, &EmptyHintCaller)
        .unwrap();

    // Dump artifacts to disk (Criterion runs in target directory, paths are relative)
    let circuit_file = "bench_circuit.txt".to_string();
    let witness_file = "bench_witness.txt".to_string();

    {
        let file = std::fs::File::create(&circuit_file).unwrap();
        let writer = std::io::BufWriter::new(file);
        compile_result
            .layered_circuit
            .serialize_into(writer)
            .unwrap();
    }

    {
        let file = std::fs::File::create(&witness_file).unwrap();
        let writer = std::io::BufWriter::new(file);
        witness.serialize_into(writer).unwrap();
    }

    (circuit_file, witness_file)
}

pub fn prove(
    circuit_file: &str,
    witness_file: &str,
    mpi_config: MPIConfig<'_>,
) -> (M31Ext3, Proof) {
    // Load circuit & witness
    let (mut circuit, mut window) =
        Circuit::<M31x1Config>::prover_load_circuit::<M31SingleConfig>(circuit_file, &mpi_config);
    circuit.prover_load_witness_file(witness_file, &mpi_config);

    let proof = expander_prove::<M31SingleConfig>(&mut circuit, mpi_config.clone());

    // Clean up shared memory
    circuit.discard_control_of_shared_mem();
    mpi_config.free_shared_mem(&mut window);

    proof
}

pub fn verify(
    circuit_file: &str,
    witness_file: &str,
    proof: &Proof,
    claimed_v: &M31Ext3,
    mpi_config: MPIConfig<'_>,
) {
    let mut circuit =
        Circuit::<M31x1Config>::verifier_load_circuit::<M31SingleConfig>(circuit_file);
    circuit.verifier_load_witness_file(witness_file, &mpi_config);

    let verifier = Verifier::<M31SingleConfig>::new(mpi_config);
    assert!(expander_verify::<M31SingleConfig>(
        &mut circuit,
        verifier.mpi_config,
        proof,
        claimed_v
    ));
}
