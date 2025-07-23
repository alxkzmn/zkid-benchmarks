use k256::{
    ecdsa::{signature::Verifier, Signature, VerifyingKey},
    elliptic_curve::sec1::EncodedPoint,
    Secp256k1,
};
use serde::{Deserialize, Serialize};

extern crate alloc;
use alloc::vec::Vec;

#[derive(Clone, Serialize, Deserialize)]
pub struct EcdsaVerifyInput {
    pub encoded_point: EncodedPoint<Secp256k1>,
    pub message: Vec<u8>,
    pub signature: Signature,
}

pub fn ecdsa_verify(input: EcdsaVerifyInput) -> bool {
    let verifying_key: VerifyingKey =
        VerifyingKey::from_encoded_point(&input.encoded_point).unwrap();

    verifying_key
        .verify(&input.message, &input.signature)
        .is_ok()
}
