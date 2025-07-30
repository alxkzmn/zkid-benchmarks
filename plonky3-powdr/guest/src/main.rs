#![allow(unused_imports)]

use powdr_riscv_runtime;
use sha2::{Digest, Sha256};

fn sha2(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    Into::<[u8; 32]>::into(result)
}

fn main() {
    let msg = &[5u8; 2048];
    let _ = sha2(msg);
}
