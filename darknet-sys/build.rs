extern crate cc;
extern crate bindgen;

use std::env;
use std::path::{PathBuf, Path};
use std::process::{Command};

/// Generate Rust FFI bindings to the C library
fn bindgen_darknet() {
    let bindings = bindgen::Builder::default()
        .header("src/custom.h")
        .header("../../darknet/include/darknet.h")
        .generate()
        .expect("unable to generate darknet bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("unable to write darknet bindings");
}

/// Build the static library
fn build_darknet() {
    panic!("we are using a prebuild libdarknet.a");
}

fn try_to_find_and_link_lib() -> bool {
    //println!("xxxxxx {:?} {:?}", env::var("DARKNET_LIB_DIR"), env::var_os("DARKNET_STATIC"));
    if let Ok(lib_dir) = env::var("DARKNET_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", lib_dir);
        let mode = match env::var_os("DARKNET_STATIC") {
            Some(_) => "static",
            None => "dylib",
        };
        println!("cargo:rustc-link-lib={}=darknet", mode);
        return true;
    }
    false
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../darknet/");

    // Generate Rust bindings to the C library
    bindgen_darknet();

    // Check if library is available and can be linked
    if !try_to_find_and_link_lib() {
        panic!("cant find lib");
    }
}
