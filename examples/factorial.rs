use racket_sys::*;
use std::{env::args_os, ffi::CString};

fn main() {
    let collects = std::env::var("RKT_COLLECTS_DIR");

    unsafe {
        let mut ba = racket_boot_arguments_t::default();
        let prefix = format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/target/debug/");
        let petite = CString::new(prefix.clone() + "petite.boot").unwrap();
        let scheme = CString::new(prefix.clone() + "scheme.boot").unwrap();
        let racket = CString::new(prefix.clone() + "racket.boot").unwrap();
        ba.boot1_path = petite.into_raw();
        ba.boot2_path = scheme.into_raw();
        ba.boot3_path = racket.into_raw();

        let default_collect_dir = "C:\\Program Files\\Racket\\collects";
        let collects = CString::new(collects.unwrap_or_else(|_| {
            println!(
                "WARNING: env variable `RKT_COLLECTS_DIR` not set, default to `{}`",
                default_collect_dir
            );
            default_collect_dir.to_string()
        }))
        .unwrap();
        ba.collects_dir = collects.into_raw();

        let mut argv = args_os();
        let argv = argv.next().unwrap().to_string_lossy().to_string();
        ba.exec_file = CString::new(argv).unwrap().into_raw();

        racket_boot(&mut ba);

        let ver = VERSION.iter().map(|&c| c as char).collect::<String>();
        println!("Running on Chez Scheme {}", ver);

        let mod_path = CString::new("examples/factorial.rkt").unwrap();
        let mod_path_str = Sstring(mod_path.into_raw());
        let func_name = CString::new("fact").unwrap();
        let func_sym = Sstring_to_symbol(func_name.into_raw());

        let func = Scar(racket_dynamic_require(mod_path_str, func_sym));
        let x = Sinteger(5);

        let result = Scar(racket_apply(func, Scons(x, Snil)));

        let display_name = CString::new("display").unwrap();
        let display = racket_primitive(display_name.into_raw());

        let txt = CString::new("f(5)=").unwrap();
        Scall1(display, Sstring(txt.into_raw()));
        Scall1(display, result);

        let newline_name = CString::new("newline").unwrap();
        let newline = racket_primitive(newline_name.into_raw());
        Scall0(newline);
    }
}
