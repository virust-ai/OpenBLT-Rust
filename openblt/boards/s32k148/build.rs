use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Configure cc build for startup code
    cc::Build::new()
        .file("startup/startup_S32K148.S")
        .file("startup/system_S32K148.c")
        .include("startup")
        .include("include")
        .include("include/S32K148")
        .opt_level(3)
        .warnings(true)
        .extra_warnings(true)
        .compile("startup");

    // Configure cc build for clock configuration
    cc::Build::new()
        .file("src/c/clock_config.c")
        .include("include")
        .include("include/S32K148")
        .opt_level(3)
        .warnings(true)
        .extra_warnings(true)
        .compile("clock_config");

    // Tell Cargo to rerun this if any of these files change
    println!("cargo:rerun-if-changed=startup/startup_S32K148.S");
    println!("cargo:rerun-if-changed=startup/system_S32K148.c");
    println!("cargo:rerun-if-changed=startup/system_S32K148.h");
    println!("cargo:rerun-if-changed=src/c/clock_config.c");
    println!("cargo:rerun-if-changed=include/device_registers.h");
    println!("cargo:rerun-if-changed=include/S32K148.h");
    println!("cargo:rerun-if-changed=linker/S32K148_256_flash.ld");
} 
