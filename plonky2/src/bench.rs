use plonky2::{
    field::goldilocks_field::GoldilocksField,
    iop::witness::{PartialWitness, WitnessWrite},
    plonk::{
        circuit_builder::CircuitBuilder,
        circuit_data::{CircuitConfig, CircuitData, ProverCircuitData, VerifierCircuitData},
        config::{GenericConfig, PoseidonGoldilocksConfig},
        proof::ProofWithPublicInputs,
    },
};
use rand::Rng;
use sha2::{Digest, Sha256};

use crate::circuit::{array_to_bits, make_circuits};

const D: usize = 2;
type C = PoseidonGoldilocksConfig;
type F = <C as GenericConfig<D>>::F;

pub fn verify(data: &VerifierCircuitData<F, C, D>, proof: ProofWithPublicInputs<F, C, D>) {
    data.verify(proof).unwrap()
}

pub fn prove(
    data: &ProverCircuitData<F, C, D>,
    pw: PartialWitness<F>,
) -> ProofWithPublicInputs<GoldilocksField, C, D> {
    data.prove(pw).unwrap()
}

pub fn sha256_no_lookup_prepare() -> (CircuitData<F, C, D>, PartialWitness<F>) {
    const MSG_BYTE_SIZE: usize = 2048;
    let mut msg = vec![0; MSG_BYTE_SIZE];
    let mut rng = rand::thread_rng();
    for msg_bit in msg.iter_mut().take(MSG_BYTE_SIZE - 1) {
        *msg_bit = rng.gen_range(0..=1);
    }

    let mut hasher = Sha256::new();
    hasher.update(&msg);
    let hash = hasher.finalize();
    // println!("Hash: {:#04X}", hash);

    let msg_bits = array_to_bits(&msg);
    let len = msg.len() * 8;
    println!("block count: {}", (len + 65).div_ceil(512));
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;
    let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_recursion_config());
    let targets = make_circuits(&mut builder, len as u64);
    let mut pw = PartialWitness::new();

    for (i, msg_bit) in msg_bits.iter().enumerate().take(len) {
        pw.set_bool_target(targets.message[i], *msg_bit).unwrap();
    }

    let expected_res = array_to_bits(hash.as_slice());
    for (i, expected_res_bit) in expected_res.iter().enumerate() {
        if *expected_res_bit {
            builder.assert_one(targets.digest[i].target);
        } else {
            builder.assert_zero(targets.digest[i].target);
        }
    }

    println!(
        "Constructing inner proof with {} gates",
        builder.num_gates()
    );
    (builder.build::<C>(), pw)
}
