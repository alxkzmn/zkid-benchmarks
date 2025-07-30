use provekit::{bench_sha256, setup};

fn main() {
    setup().unwrap();
    bench_sha256();
}
