use std::path::{Path, PathBuf};
use std::{env, fs};

#[cfg(feature = "bundled")]
fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = {
        let mut path = PathBuf::from(env::var("OUT_DIR").unwrap());
        _ = path.pop() && path.pop() && path.pop();
        path
    };
    let bundle_dir = out_dir.join("bundled");

    if fs::metadata(&bundle_dir).is_err() {
        let url = "https://github.com/rn7s2/racket-bundled.git";
        match git2::Repository::clone(url, &bundle_dir) {
            Ok(_) => (),
            Err(e) => {
                _ = fs::remove_dir_all(&bundle_dir);
                panic!("Failed to clone: {}", e);
            }
        };
    }

    // link libraries
    println!("cargo:rustc-link-search=native=m");

    let headers;
    if cfg!(windows) {
        let lib_path = {
            let mut path = bundle_dir.clone();
            path.push("windows");
            path
        };

        // link dynamic library
        println!("cargo:rustc-link-search={}", lib_path.display());
        println!("cargo:rustc-link-lib=libracketcs_dg1etc");

        headers = {
            let path = lib_path.clone();
            let cs_h = path.join("chezscheme.h");
            let rkt_h = path.join("racketcs.h");
            (
                cs_h.to_str().unwrap().to_string(),
                rkt_h.to_str().unwrap().to_string(),
            )
        };
    } else if cfg!(target_os = "macos") {
        let lib_path = {
            let mut path = bundle_dir.clone();
            path.push("macos");
            path
        };

        // link library
        println!("cargo:rustc-link-search={}", lib_path.display());
        println!("cargo:rustc-link-lib=racketcs");

        println!("cargo:rustc-link-lib=iconv");
        println!("cargo:rustc-link-lib=ncurses");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");

        headers = {
            let path = lib_path.clone();
            let cs_h = path.join("chezscheme.h");
            let rkt_h = path.join("racketcs.h");
            (
                cs_h.to_str().unwrap().to_string(),
                rkt_h.to_str().unwrap().to_string(),
            )
        };
    } else {
        let lib_path = {
            let mut path = bundle_dir.clone();
            path.push("linux");
            path
        };
        println!("cargo:rustc-link-search={}", lib_path.display());
        println!("cargo:rustc-link-lib=racketcs");

        println!("cargo:rustc-link-lib=ncurses");
        println!("cargo:rustc-link-lib=lz4");
        println!("cargo:rustc-link-lib=z");

        headers = {
            let path = lib_path.clone();
            let cs_h = path.join("chezscheme.h");
            let rkt_h = path.join("racketcs.h");
            (
                cs_h.to_str().unwrap().to_string(),
                rkt_h.to_str().unwrap().to_string(),
            )
        };
    }

    // generate bindings
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
    let boot_files = ["petite.boot", "scheme.boot", "racket.boot"];
    if cfg!(windows) {
        let dll_path = {
            let mut path = bundle_dir.clone();
            path.push("windows");
            path.push("libracketcs_dg1etc.dll");
            path
        };

        fs::copy(
            dll_path,
            out_dir.to_str().unwrap().to_string() + "/libracketcs_dg1etc.dll",
        )
        .expect("Failed to copy dll file.");

        // copy boot files
        for boot_file in boot_files.iter() {
            let boot_path = {
                let mut path = bundle_dir.clone();
                path.push("windows");
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
        // copy boot files
        for boot_file in boot_files.iter() {
            let boot_path = {
                let mut path = bundle_dir.clone();
                path.push("macos");
                path.push(boot_file);
                path
            };

            fs::copy(
                boot_path,
                out_dir.to_str().unwrap().to_string() + "/" + boot_file,
            )
            .expect("Failed to copy boot file.");
        }

        // copy framework
        let mut framework_dir = bundle_dir.clone();
        framework_dir.push("macos");
        framework_dir.push("Racket.framework");

        let mut out_dir = out_dir.clone();
        out_dir.pop();
        out_dir.pop();
        copy_recursively(
            framework_dir,
            out_dir.to_str().unwrap().to_string() + "/Racket.framework",
        )
        .expect("Failed to copy Racket framework.");
    } else {
        let bootfile_dir = {
            let mut path = bundle_dir.clone();
            path.push("linux");
            path
        };
        for boot_file in boot_files.iter() {
            let boot_path = {
                let mut path = bootfile_dir.clone();
                path.push(boot_file);
                path
            };

            fs::copy(
                boot_path,
                out_dir.to_str().unwrap().to_string() + "/" + boot_file,
            )
            .expect("Failed to copy boot file.");
        }
    }
}

#[cfg(not(feature = "bundled"))]
fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    const DEFAULT_RKT_VER: &str = "8.13";

    // directories
    let rkt_home = {
        let home = env::var("RACKET_CS_HOME");
        if cfg!(windows) {
            PathBuf::from(home.unwrap_or("C:/Program Files/Racket".to_string()))
        } else if cfg!(target_os = "macos") {
            PathBuf::from(home.unwrap_or("/Applications/Racket v".to_string() + DEFAULT_RKT_VER))
        } else {
            PathBuf::from(home.unwrap_or("/usr/".to_string()))
        }
    };
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

        let lib_tool_exe =
            locate_vs_lib_tool().expect("Failed to locate lib.exe from VS installation.");
        let status = std::process::Command::new(lib_tool_exe)
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
    } else if cfg!(target_os = "macos") {
        let version = env::var("RACKET_CS_VERSION").unwrap_or(DEFAULT_RKT_VER.to_string());
        let lib_path = {
            let mut path = rkt_home.clone();
            path.push("lib");
            path.push("Racket.framework");
            path.push("Versions");
            path.push(version + "_CS");
            path.push("Racket");
            path
        };
        let dest_lib_path = {
            let mut path = out_dir.clone();
            path.push("libracketcs.a");
            path
        };
        fs::copy(lib_path, dest_lib_path).expect("Failed to find Racket library.");

        // link library
        println!("cargo:rustc-link-search={}", out_dir.display());
        println!("cargo:rustc-link-lib=racketcs");

        println!("cargo:rustc-link-lib=iconv");
        println!("cargo:rustc-link-lib=ncurses");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
    } else {
        let lib_dir = {
            let mut path = rkt_home.clone();
            path.push("lib");
            path
        };
        println!("cargo:rustc-link-search={}", lib_dir.display());
        println!("cargo:rustc-link-lib=racketcs");

        println!("cargo:rustc-link-lib=ncurses");
        println!("cargo:rustc-link-lib=lz4");
        println!("cargo:rustc-link-lib=z");
    }

    // generate bindings
    let headers = {
        let mut path = rkt_home.clone();
        path.push("include");
        if cfg!(target_os = "linux") {
            path.push("racket");
        }

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
    let boot_files = ["petite.boot", "scheme.boot", "racket.boot"];
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

        // copy boot files
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
        // copy boot files
        let version = env::var("RACKET_CS_VERSION").unwrap_or(DEFAULT_RKT_VER.to_string());
        for boot_file in boot_files.iter() {
            let boot_path = {
                let mut path = rkt_home.clone();
                path.push("lib");
                path.push("Racket.framework");
                path.push("Versions");
                path.push(version.clone() + "_CS");
                path.push("boot");
                path.push(boot_file);
                path
            };

            fs::copy(
                boot_path,
                out_dir.to_str().unwrap().to_string() + "/" + boot_file,
            )
            .expect("Failed to copy boot file.");
        }

        // copy framework
        let mut framework_dir = rkt_home.clone();
        framework_dir.push("lib");
        framework_dir.push("Racket.framework");

        let mut out_dir = out_dir.clone();
        out_dir.pop();
        out_dir.pop();
        copy_recursively(
            framework_dir,
            out_dir.to_str().unwrap().to_string() + "/Racket.framework",
        )
        .expect("Failed to copy Racket framework.");
    } else {
        let bootfile_dir = {
            let mut path = rkt_home.clone();
            path.push("lib");
            path.push("racket");
            path
        };
        for boot_file in boot_files.iter() {
            let boot_path = {
                let mut path = bootfile_dir.clone();
                path.push(boot_file);
                path
            };

            fs::copy(
                boot_path,
                out_dir.to_str().unwrap().to_string() + "/" + boot_file,
            )
            .expect("Failed to copy boot file.");
        }
    }
}

/// Copy files from source to destination recursively.
/// thanks: https://nick.groenen.me/notes/recursively-copy-files-in-rust/
fn copy_recursively(
    source: impl AsRef<Path>,
    destination: impl AsRef<Path>,
) -> std::io::Result<()> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn locate_vs_lib_tool() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        let mut vcvars = vcvars::Vcvars::new();
        let path = vcvars.get_cached("PATH").unwrap().to_string();
        let dirs = path.split(";").collect::<Vec<_>>();
        for dir in &dirs {
            let mut exe = PathBuf::from(dir);
            exe.push("lib.exe");
            if exe.exists() {
                return Some(exe);
            }
        }
    }
    None
}
