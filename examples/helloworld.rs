use racket_sys::*;
use std::{env::args_os, ffi::CString};

fn main() {
    unsafe {
        let mut ba = racket_boot_arguments_t::default();
        let prefix = format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/target/debug/");
        let petite = CString::new(prefix.clone() + "petite.boot").unwrap();
        let scheme = CString::new(prefix.clone() + "scheme.boot").unwrap();
        let racket = CString::new(prefix.clone() + "racket.boot").unwrap();
        ba.boot1_path = petite.into_raw();
        ba.boot2_path = scheme.into_raw();
        ba.boot3_path = racket.into_raw();

        let mut argv = args_os();
        let argv = argv.next().unwrap().to_string_lossy().to_string();
        ba.exec_file = CString::new(argv).unwrap().into_raw();

        racket_boot(&mut ba);

        let ver = VERSION.iter().map(|&c| c as char).collect::<String>();
        println!("Running on Chez Scheme {}", ver);

        let func_name = CString::new("display").unwrap();
        let func = racket_primitive(func_name.into_raw());

        let str = CString::new("Hello, world!").unwrap();
        let arg = Sstring(str.into_raw());
        Scall1(func, arg);

        let func_name = CString::new("newline").unwrap();
        let func = racket_primitive(func_name.into_raw());
        Scall0(func);
    }
}
