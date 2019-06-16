extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;

const RPC_VERSION: &'static str = "3.1.0";

fn main() {
    // compiles the RPC library
    let install_path = cmake::Config::new(format!("discord-rpc-{}", RPC_VERSION)).build();

    println!(
        "cargo:rustc-link-search={}",
        install_path.join("lib").display()
    );
    println!(
        "cargo:rustc-link-search={}",
        install_path.join("lib64").display()
    );
    let include_path = format!("discord-rpc-{}/include", RPC_VERSION);

    // generates the bindings to the RPC headers
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", include_path))
        .generate()
        .expect("Unable to generate bindings");

    // writes the generated bindings
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=static=discord-rpc");
    //println!("cargo:rustc-flags=-l dylib=stdc++");
}
