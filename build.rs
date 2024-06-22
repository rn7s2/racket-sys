use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // directories
    let rkt_home =
        PathBuf::from(env::var("RACKET_CS_HOME").unwrap_or("C:/Program Files/Racket".to_string()));
    let out_dir = {
        let mut path = PathBuf::from(env::var("OUT_DIR").unwrap());
        _ = path.pop() && path.pop() && path.pop();
        path
    };

    // link libraries
    println!("cargo:rustc-link-search=native=m");
    if cfg!(windows) {
        // generate lib file from module definition file
        let def_path = {
            let mut path = rkt_home.clone();
            path.push("lib");
            path.push("libracketcs_dg1etc.def");
            path
        };
        let lib_path = {
            let mut path = out_dir.clone();
            path.push("libracketcs_dg1etc.lib");
            path
        };

        let status = Command::new("lib")
            .arg(&format!("/def:{}", def_path.to_str().unwrap()))
            .arg(&format!("/out:{}", lib_path.to_str().unwrap()))
            .arg("/machine:x64")
            .status()
            .expect("Failed to generate lib file from module def.");
        if status.code() != Some(0) {
            panic!("Failed to generate lib file from module def.");
        }

        // link dynamic library
        println!("cargo:rustc-link-search={}", out_dir.display());
        println!("cargo:rustc-link-lib=libracketcs_dg1etc");

        // cc::Build::new().file("empty.c").compile("racket-sys");
    } else if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        panic!("Platform is not supported yet!");
    } else {
        panic!("Platform is not supported yet!");
    }

    // generate bindings
    let headers = {
        let mut path = rkt_home.clone();
        path.push("include");

        let cs_h = path.join("chezscheme.h");
        let rkt_h = path.join("racketcs.h");
        (
            cs_h.to_str().unwrap().to_string(),
            rkt_h.to_str().unwrap().to_string(),
        )
    };
    let bindings = bindgen::Builder::default()
        .header(headers.0)
        .header(headers.1)
        .generate()
        .expect("Unable to generate bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // copy dependencies to output directory
    // works even if this crate is a dependency of another crate
    if cfg!(windows) {
        // generate lib file from module definition file
        let dll_path = {
            let mut path = rkt_home.clone();
            path.push("lib");
            path.push("libracketcs_dg1etc.dll");
            path
        };

        fs::copy(
            dll_path,
            out_dir.to_str().unwrap().to_string() + "/libracketcs_dg1etc.dll",
        )
        .expect("Failed to copy dll file.");

        let boot_files = ["petite.boot", "scheme.boot", "racket.boot"];
        for boot_file in boot_files.iter() {
            let boot_path = {
                let mut path = rkt_home.clone();
                path.push("lib");
                path.push(boot_file);
                path
            };

            fs::copy(
                boot_path,
                out_dir.to_str().unwrap().to_string() + "/" + boot_file,
            )
            .expect("Failed to copy boot file.");
        }
    } else if cfg!(target_os = "macos") {
        panic!("Platform is not supported yet!");
    } else {
        panic!("Platform is not supported yet!");
    }
}
