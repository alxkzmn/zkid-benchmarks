#![cfg_attr(feature = "guest", no_std)]
#![no_main]

use guests::sha2;

#[jolt::provable]
fn sha2(input: &[u8]) -> [u8; 32] {
    sha2::sha2(input)
}
