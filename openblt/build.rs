use std::env;
use std::path::PathBuf;

fn main() {
    // Get the output directory for build artifacts
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/");
} 
