use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        File::create(out.join("device.x"))
            .unwrap()
            .write_all(include_bytes!("device.x"))
            .unwrap();
        println!("cargo:rustc-link-search={}", out.display());
        println!("cargo:rerun-if-changed=device.x");
    }
    let target = env::var_os("TARGET").unwrap();
    match target.to_str().unwrap() {
        "thumbv7em-none-eabihf" => println!("cargo:rustc-cfg=cm4"),
        "thumbv6m-none-eabi" => println!("cargo:rustc-cfg=cm0"),
        _ => {
            panic!("Only thumbv7em-none-eabihf and thumbv6m-none-eabi target triple are supported")
        }
    }
    println!("cargo:rerun-if-changed=build.rs");
}
