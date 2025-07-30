use guests::ecdsa::EcdsaVerifyInput;
use k256::{ecdsa::Signature, elliptic_curve::sec1::EncodedPoint, Secp256k1};
use rand::{rngs::StdRng, RngCore, SeedableRng};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde::Serialize;

pub mod bench;
pub mod metadata;

pub fn sha2_input(num_bytes: usize) -> Vec<u8> {
    let mut rng = StdRng::seed_from_u64(1337);
    let mut message = vec![0; num_bytes];
    rng.fill_bytes(&mut message);
    message
}

pub fn ecdsa_input() -> EcdsaVerifyInput {
    const MESSAGE: &[u8] = include_bytes!("../../utils/ecdsa_signature/message.txt");
    const KEY: &[u8] = include_bytes!("../../utils/ecdsa_signature/verifying_key.txt");
    const SIGNATURE: &[u8] = include_bytes!("../../utils/ecdsa_signature/signature.txt");

    // Use a static variable to store the decoded message so it has a 'static lifetime
    let message = hex::decode(MESSAGE).expect("Failed to decode hex of 'message'");

    let encoded_point = EncodedPoint::<Secp256k1>::from_bytes(
        hex::decode(KEY).expect("Failed to decode hex of 'verifying_key'"),
    )
    .expect("Invalid encoded verifying_key bytes");

    let bytes = hex::decode(SIGNATURE).expect("Failed to decode hex of 'signature'");
    let signature = Signature::from_slice(&bytes).expect("Invalid signature bytes");

    EcdsaVerifyInput {
        encoded_point,
        message: message.clone(),
        signature,
    }
}

pub fn load_elf(path: &str) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|err| {
        panic!("Failed to load ELF file from {}: {}", path, err);
    })
}

pub fn size<T: Serialize>(item: &T) -> usize {
    bincode::serialized_size(item).unwrap() as usize
}

pub fn write_json<T: Serialize>(data: &T, output_path: &str) {
    let json_data = serde_json::to_string_pretty(&data).expect("Failed to serialize to JSON");
    let path = Path::new(&output_path);

    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(json_data.as_bytes())
        .expect("Failed to write to file");
}
