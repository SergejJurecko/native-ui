extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;
use std::ffi::OsStr;

fn main() {
    let target = env::var("TARGET").unwrap();
    let mut cc = cc::Build::new();

    for entry in ::std::fs::read_dir("libui/common").unwrap() {
        if let Ok(entry) = entry {
            if entry.path().extension() == Some(OsStr::new("c")) {
                cc.file(entry.path());
            }
        }
    }
    cc.flag("--std=c99")
    .warnings(false)
    .extra_warnings(false)
    .flag("-Wno-deprecated");
    // .flag("--std=c++11")
    // .flag("-O2");

    if target.contains("apple") {
        cc.flag("-mmacosx-version-min=10.8");
        // cc.flag("-Wall")
            // .flag("-Wextra")
            // .flag("-pedantic")
            // .flag("-Wno-unused-parameter")
            // .flag("-Wno-switch")
            // .flag("-fvisibility=hidden");
        for entry in ::std::fs::read_dir("libui/darwin").unwrap() {
            if let Ok(entry) = entry {
                if entry.path().extension() == Some(OsStr::new("m")) {
                    cc.file(entry.path());
                }
            }
        }
        println!("cargo:rustc-link-lib=framework=Cocoa");
    }

    cc.compile("libui.a");
    // println!("cargo:rustc-link-lib=bz2");

    let bindings = bindgen::Builder::default()
        .header("libui/ui.h")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
