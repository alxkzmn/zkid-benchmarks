use hex;
use k256::{
    ecdsa::{signature::Signer, Signature, SigningKey},
    elliptic_curve::rand_core::OsRng,
};
use std::{fs::File, io::Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let message = b"This is a message that will be signed, and verified within the zkVM".to_vec();
    let signing_key = SigningKey::random(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    let signature: Signature = signing_key.sign(&message);

    let mut message_file = File::create("ecdsa_signature/message.txt")?;
    let mut verifying_key_file = File::create("ecdsa_signature/verifying_key.txt")?;
    let mut signature_file = File::create("ecdsa_signature/signature.txt")?;

    write!(message_file, "{}", hex::encode(&message))?;
    write!(
        verifying_key_file,
        "{}",
        hex::encode(verifying_key.to_encoded_point(false).as_bytes())
    )?;
    write!(signature_file, "{}", hex::encode(signature.to_bytes()))?;

    println!("message, verifying_key, and signature have been written to files");
    Ok(())
}
