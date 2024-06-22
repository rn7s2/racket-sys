use racket_sys::*;
use std::{env::args_os, ffi::CString, ptr::null};

fn main() {
    unsafe {
        let mut ba = racket_boot_arguments_t {
            boot1_path: null(),
            boot1_offset: 0,
            boot1_len: 0,
            boot2_path: null(),
            boot2_offset: 0,
            boot2_len: 0,
            boot3_path: null(),
            boot3_offset: 0,
            boot3_len: 0,
            argc: 0,
            argv: 0 as *mut *mut ::std::os::raw::c_char,
            exec_file: null(),
            run_file: null(),
            collects_dir: null(),
            config_dir: null(),
            dll_dir: 0 as *mut ::std::os::raw::c_void,
            k_file: null(),
            cs_compiled_subdir: 0,
            segment_offset: 0,
            dll_open: 0 as *mut ::std::os::raw::c_void,
            dll_find_object: 0 as *mut ::std::os::raw::c_void,
            dll_close: 0 as *mut ::std::os::raw::c_void,
            exit_after: 0,
            is_gui: 0,
            wm_is_gracket_or_x11_arg_count: 0,
            gracket_guid_or_x11_args: 0 as *mut i8,
        };

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
