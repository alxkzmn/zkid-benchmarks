// Most of the code borrowed from powdr/src/lib.rs

use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use powdr::{GoldilocksField, Pipeline, Session, riscv};

fn pil_file_path(asm_name: &Path) -> PathBuf {
    let file_stem = asm_name.file_stem().unwrap().to_str().unwrap();
    let opt_file_stem = format!("{file_stem}_opt");
    asm_name.with_file_name(opt_file_stem).with_extension("pil")
}

pub fn prepare_pipeline() -> powdr::Pipeline<powdr::GoldilocksField> {
    let session = Session::builder()
        .guest_path("./guest")
        .out_path("powdr-target")
        .chunk_size_log2(18)
        .build();

    let mut pipeline = session.into_pipeline();

    let asm_name = pipeline.asm_string().unwrap().0.clone().unwrap();
    let pil_file = pil_file_path(&asm_name);

    let generate_artifacts = if let Ok(existing_pil) = fs::read_to_string(&pil_file) {
        let computed_pil = pipeline.compute_optimized_pil().unwrap().to_string();
        if existing_pil != computed_pil {
            log::info!("Compiled PIL changed, invalidating artifacts...");
            true
        } else {
            log::info!("Compiled PIL did not change, will try to reuse artifacts...");
            false
        }
    } else {
        log::info!("PIL file not found, will generate artifacts...");
        true
    };

    let out_path = Path::new("powdr-target");
    let pkey = out_path.join("pkey.bin");
    let vkey = out_path.join("vkey.bin");

    if generate_artifacts {
        println!("Creating program ZK setup. This has to be done only once per program.");
        pipeline.compute_fixed_cols().unwrap();
        pipeline.setup_backend().unwrap();
        export_setup(&mut pipeline);
        pipeline.set_pkey_file(pkey.clone());
        pipeline.set_vkey_file(vkey.clone());
    } else {
        println!("Loading program ZK setup.");
        if pipeline.read_constants_mut(out_path).is_ok() {
            println!("Read constants from file...");
        } else {
            pipeline.compute_fixed_cols().unwrap();
        }

        if pkey.exists() && vkey.exists() {
            println!("Re-using proving and verification keys...");
            pipeline.set_pkey_file(pkey.clone());
            pipeline.set_vkey_file(vkey.clone());
            pipeline.setup_backend().unwrap();
        } else {
            println!("Exporting setup...");
            export_setup(&mut pipeline);
            pipeline.set_pkey_file(pkey.clone());
            pipeline.set_vkey_file(vkey.clone());
        }
    }

    pipeline
}

fn export_setup(pipeline: &mut powdr::Pipeline<powdr::GoldilocksField>) {
    let mut path = PathBuf::from("powdr-target");
    path.push("pkey.bin");
    let file = File::create(path).unwrap();

    pipeline.export_proving_key(file).unwrap();

    let mut path = PathBuf::from("powdr-target");
    path.push("vkey.bin");
    let file = File::create(path).unwrap();

    pipeline.export_verification_key(file).unwrap();
}

pub fn prove(pipeline: &mut powdr::Pipeline<powdr::GoldilocksField>) {
    let bootloader_inputs =
        riscv::continuations::rust_continuations_dry_run(&mut pipeline.clone(), None);

    let generate_proof = |pipeline: &mut Pipeline<GoldilocksField>| -> Result<(), Vec<String>> {
        pipeline.compute_witness()?;
        pipeline.compute_proof().unwrap();
        //println!("Proof size: {} MB", proof.len() as f64 / 1024.0 / 1024.0);
        Ok(())
    };

    pipeline.rollback_from_witness();

    riscv::continuations::rust_continuations(pipeline, generate_proof, bootloader_inputs).unwrap();
}

pub fn verify(mut pipeline: powdr::Pipeline<GoldilocksField>) {
    let proof = pipeline.proof().unwrap().clone();
    let publics: Vec<GoldilocksField> = pipeline
        .publics()
        .unwrap()
        .iter()
        .map(|(_name, v)| v.expect("all publics should be known since we created a proof"))
        .collect();
    pipeline.verify(&proof, &[publics]).unwrap();
}
