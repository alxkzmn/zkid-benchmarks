use anyhow::Error;
use binius::bench::{prove, sha256_with_lookup_prepare, verify};
use utils::bench::measure_peak_memory;

fn main() -> Result<(), Error> {
    let allocator = bumpalo::Bump::new();

    let ((constraint_system, args, witness, backend), peak_memory) =
        measure_peak_memory(|| sha256_with_lookup_prepare(&allocator));

    println!(
        "Preprocessing(lookup) peak memory: {} MB",
        peak_memory as f32 / (1024.0 * 1024.0),
    );

    let ((cs, args, proof), peak_memory) =
        measure_peak_memory(|| prove(constraint_system, args, witness, backend));

    println!(
        "Proving(lookup) peak memory: {} MB",
        peak_memory as f32 / (1024.0 * 1024.0)
    );

    verify(cs, args, proof);

    Ok(())
}
