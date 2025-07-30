use noir_native_sha256::ryan_sha256_noir;

fn main(input: [u8; 4096], input_len: u64) -> pub [u8; 32] {
    ryan_sha256_noir::sha256_var(input, input_len)
} 