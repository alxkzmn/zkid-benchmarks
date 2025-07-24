#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use anyhow::Error;
use binius::bench::{prove, sha256_no_lookup_prepare, sha256_with_lookup_prepare};
use binius_utils::{SerializationMode, SerializeBytes};
use jemalloc_ctl::{
    epoch,
    stats::{self},
};

fn main() -> Result<(), Error> {
    // No-lookup sha256
    epoch::advance().unwrap();
    let resident_before = stats::resident::read().unwrap();

    let allocator = bumpalo::Bump::new();
    let (constraint_system, args, witness, backend) = sha256_no_lookup_prepare(&allocator);

    epoch::advance().unwrap();
    let resident_after = stats::resident::read().unwrap();

    println!(
        "Preprocessing: resident memory (no lookup): {} MB",
        (resident_after - resident_before) as f32 / 1024.0 / 1024.0,
    );

    let mut write_buf = vec![];
    constraint_system
        .serialize(&mut write_buf, SerializationMode::Native)
        .unwrap();
    let len = write_buf.len();
    println!(
        "Constraint system size (no lookup): {} KB",
        len as f32 / 1024.0
    );

    epoch::advance().unwrap();
    let resident_before = stats::resident::read().unwrap();

    let (_, _, _) = prove(constraint_system, args, witness, backend);

    epoch::advance().unwrap();
    let resident_after = stats::resident::read().unwrap();
    println!(
        "Proving: resident memory (no lookup): {} MB",
        (resident_after - resident_before) as f32 / 1024.0 / 1024.0,
    );

    //Lookup sha256
    epoch::advance().unwrap();
    let resident_before = stats::resident::read().unwrap();

    let allocator = bumpalo::Bump::new();
    let (constraint_system, args, witness, backend) = sha256_with_lookup_prepare(&allocator);

    epoch::advance().unwrap();
    let resident_after = stats::resident::read().unwrap();

    println!(
        "Preprocessing: resident memory (with lookup): {} MB",
        (resident_after - resident_before) as f32 / 1024.0 / 1024.0,
    );

    let mut write_buf = vec![];
    constraint_system
        .serialize(&mut write_buf, SerializationMode::Native)
        .unwrap();
    let len = write_buf.len();
    println!(
        "Constraint system size (with lookup): {} KB",
        len as f32 / 1024.0
    );

    epoch::advance().unwrap();
    let resident_before = stats::resident::read().unwrap();

    let (_, _, _) = prove(constraint_system, args, witness, backend);

    epoch::advance().unwrap();
    let resident_after = stats::resident::read().unwrap();
    println!(
        "Proving: resident memory (with lookup): {} MB",
        (resident_after - resident_before) as f32 / 1024.0 / 1024.0,
    );

    Ok(())
}
