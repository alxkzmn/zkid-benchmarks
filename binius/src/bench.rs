use binius_circuits::{
    builder::{ConstraintSystemBuilder, types::U},
    unconstrained::unconstrained,
};
use binius_core::{
    constraint_system::{self, ConstraintSystem, Proof},
    fiat_shamir::HasherChallenger,
    oracle::OracleId,
    tower::CanonicalTowerFamily,
    witness::MultilinearExtensionIndex,
};
use binius_field::{
    BinaryField1b, BinaryField128b, arch::OptimalUnderlier, as_packed_field::PackedType,
};
use binius_hal::{CpuBackend, make_portable_backend};
use binius_hash::groestl::{Groestl256, Groestl256ByteCompression};
use binius_utils::{checked_arithmetics::log2_ceil_usize, rayon::adjust_thread_pool};
use bytesize::ByteSize;
pub const COMPRESSION_LOG_LEN: usize = 5;

#[derive(Debug)]
pub struct Args {
    /// The number of compressions to verify.
    pub n_compressions: u32,
    /// The negative binary logarithm of the Reed–Solomon code rate.
    pub log_inv_rate: u32,
}

pub fn sha256_no_lookup_prepare(
    allocator: &bumpalo::Bump,
) -> (
    ConstraintSystem<BinaryField128b>,
    Args,
    MultilinearExtensionIndex<'_, PackedType<OptimalUnderlier, BinaryField128b>>,
    CpuBackend,
) {
    adjust_thread_pool()
        .as_ref()
        .expect("failed to init thread pool");

    let args = Args {
        n_compressions: 33,
        log_inv_rate: 1,
    };

    println!("Verifying {} sha256 compressions", args.n_compressions);

    let log_n_compressions = log2_ceil_usize(args.n_compressions as usize);

    let mut builder = ConstraintSystemBuilder::new_with_witness(allocator);

    let trace_gen_scope = tracing::info_span!("generating trace").entered();
    let input: [OracleId; 16] = array_util::try_from_fn(|i| {
        unconstrained::<BinaryField1b>(&mut builder, i, log_n_compressions + COMPRESSION_LOG_LEN)
    })
    .unwrap();

    let _state_out = binius_circuits::sha256::sha256(
        &mut builder,
        input,
        log_n_compressions + COMPRESSION_LOG_LEN,
    )
    .unwrap();
    drop(trace_gen_scope);

    let witness = builder
        .take_witness()
        .expect("builder created with witness");

    let constraint_system: ConstraintSystem<BinaryField128b> = builder.build().unwrap();

    let backend = make_portable_backend();

    (constraint_system, args, witness, backend)
}

pub fn sha256_with_lookup_prepare(
    allocator: &bumpalo::Bump,
) -> (
    ConstraintSystem<BinaryField128b>,
    Args,
    MultilinearExtensionIndex<'_, PackedType<OptimalUnderlier, BinaryField128b>>,
    CpuBackend,
) {
    adjust_thread_pool()
        .as_ref()
        .expect("failed to init thread pool");

    let args = Args {
        n_compressions: 33,
        log_inv_rate: 1,
    };

    println!(
        "Verifying {} sha256 compressions with lookups",
        args.n_compressions
    );

    let log_n_compressions = log2_ceil_usize(args.n_compressions as usize);

    let mut builder = ConstraintSystemBuilder::new_with_witness(allocator);

    let trace_gen_scope = tracing::info_span!("generating witness").entered();
    let input: [OracleId; 16] = array_util::try_from_fn(|i| {
        unconstrained::<BinaryField1b>(&mut builder, i, log_n_compressions + COMPRESSION_LOG_LEN)
    })
    .unwrap();

    let _state_out = binius_circuits::lasso::sha256(
        &mut builder,
        input,
        log_n_compressions + COMPRESSION_LOG_LEN,
    )
    .unwrap();
    drop(trace_gen_scope);

    let witness = builder
        .take_witness()
        .expect("builder created with witness");

    let constraint_system = builder.build().unwrap();

    let backend = make_portable_backend();

    (constraint_system, args, witness, backend)
}

pub fn prove(
    constraint_system: ConstraintSystem<binius_field::BinaryField128b>,
    args: Args,
    witness: MultilinearExtensionIndex<'_, PackedType<OptimalUnderlier, BinaryField128b>>,
    backend: CpuBackend,
) -> (ConstraintSystem<BinaryField128b>, Args, Proof) {
    const SECURITY_BITS: usize = 100;

    let proof = constraint_system::prove::<
        U,
        CanonicalTowerFamily,
        Groestl256,
        Groestl256ByteCompression,
        HasherChallenger<Groestl256>,
        _,
    >(
        &constraint_system,
        args.log_inv_rate as usize,
        SECURITY_BITS,
        &[],
        witness,
        &backend,
    )
    .unwrap();

    println!("Proof size: {}", ByteSize::b(proof.get_proof_size() as u64));

    (constraint_system, args, proof)
}

pub fn verify(
    constraint_system: ConstraintSystem<binius_field::BinaryField128b>,
    args: Args,
    proof: Proof,
) {
    const SECURITY_BITS: usize = 100;

    constraint_system::verify::<
        U,
        CanonicalTowerFamily,
        Groestl256,
        Groestl256ByteCompression,
        HasherChallenger<Groestl256>,
    >(
        &constraint_system,
        args.log_inv_rate as usize,
        SECURITY_BITS,
        &[],
        proof,
    )
    .unwrap();
}
