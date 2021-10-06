// extern crate cc;
// extern crate regex;
// use regex::Regex;
use std::{env, fs::File, io::Write, path::Path};

/// Put the linker script somewhere the linker can find it.
fn main() {
    // let target = env::var("TARGET").expect("TARGET not set!");
    // let target_arch = target.split('-').next().unwrap();
    // eprintln!("target_arch={}", target_arch);
    // let re = Regex::new(r"riscv(32|64)(.*)").unwrap();
    // let caps = re.captures(target_arch).unwrap();
    // eprintln!("matches={}, {}", &caps[1], &caps[2]);
    // let abi_0 = if &caps[1] == "64" { "lp64" } else { "lp32" };
    // let abi_1 = if caps[2].contains('g') || caps[2].contains('f') || caps[2].contains('d') { "d" } else { "" };
    
    
    // cc::Build::new()
    // // .file("crt.S")
    // .file("syscalls.c")
    // // .no_default_flags(true)
    // .flag(format!("-march=rv{}{}", &caps[1], &caps[2]).as_str())
    // .flag(format!("-mabi={}{}", abi_0, abi_1).as_str())
    // .flag("-mcmodel=medany")
    // .flag("-fno-common")
    // .flag("-nostdlib")
    // .flag("-Tlinker.x")
    // .flag("-nostartfiles")
    // .flag("-fno-builtin-printf")
    // .flag("-fno-tree-loop-distribute-patterns")
    // .compile("crt"); // outputs `libcrt.a`
    
    let out_dir = env::var("OUT_DIR").expect("No out dir");
    let dest_path = Path::new(&out_dir);

    let mut f = File::create(&dest_path.join("linker.x")).expect("Could not create file");
    f.write_all(include_bytes!("linker.ld")).expect("Could not write file");

    println!("cargo:rustc-link-search={}", dest_path.display());
    
    println!("cargo:rerun-if-changed=linker.ld");
    println!("cargo:rerun-if-changed=build.rs");
}
