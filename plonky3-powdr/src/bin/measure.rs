use sha::bench::{prepare_pipeline, prove, verify};
use utils::bench::measure_peak_memory;

fn main() {
    let (mut pipeline, peak_memory) = measure_peak_memory(prepare_pipeline);

    println!(
        "Preprocessing peak memory: {} GB",
        peak_memory as f32 / (1024.0 * 1024.0 * 1024.0),
    );

    let (_, peak_memory) = measure_peak_memory(|| prove(&mut pipeline));

    println!(
        "Proving peak memory: {} GB",
        peak_memory as f32 / (1024.0 * 1024.0 * 1024.0),
    );

    verify(pipeline);
}
