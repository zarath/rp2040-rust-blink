//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.

use std::env;
use std::f32::consts::PI;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn pdm_table() -> Vec<u32> {
    let mut bits: Vec<u32> = Vec::with_capacity(4096);
    let mut qe = 0.0f32;
    let mut y: f32;

    let vals = (0..4096).map(|i| (((i + 3072) as f32) / 2048. * PI).sin());
    for v in vals {
        qe += v;
        if qe > 0.0 {
            bits.push(1);
            y = 1.0;
        } else {
            bits.push(0);
            y = -1.0;
        }
        qe -= y;
    }

    bits.chunks(32)
        .map(|x| x.iter().fold(0u32, |res, b| (res << 1) ^ *b))
        .collect::<Vec<u32>>()
}

fn main() {
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    // println!("cargo:rerun-if-changed=memory.x");
    let pdm = pdm_table();
    File::create(out.join("pdm_table.rs"))
        .unwrap()
        .write(format!("const PDM_TABLE: [u32; {}] = {:?};", pdm.len(), pdm).as_bytes())
        .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
