#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{os::raw, ptr::null};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Default for racket_boot_arguments_t {
    fn default() -> Self {
        Self {
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
        }
    }
}

// some of the following code comes from amarmaduke's chez-rs binding
// thanks: https://github.com/amarmaduke/chez-rs/blob/main/chez-sys/src/bindings.rs

pub const Snil: ptr = 0x26 as ptr;
pub const Strue: ptr = 0xE as ptr;
pub const Sfalse: ptr = 0x6 as ptr;
pub const Sbwp_object: ptr = 0x4E as ptr;
pub const Seof_object: ptr = 0x36 as ptr;
pub const Svoid: ptr = 0x3E as ptr;

#[inline(always)]
pub unsafe fn inc_ptr<T: Copy>(x: ptr, by: uptr) -> T {
    *(((x as uptr) + by) as *mut T)
}

macro_rules! predicate {
    ($name:ident, $loc:literal) => {
        #[inline(always)]
        pub fn $name(x: ptr) -> bool {
            x as uptr == $loc
        }
    };

    ($name:ident, $mask:literal, $loc:literal) => {
        #[inline(always)]
        pub fn $name(x: ptr) -> bool {
            (x as uptr) & $mask == $loc
        }
    };

    ($name:ident, $mask1:literal, $loc1:literal, $loc2:literal) => {
        #[inline(always)]
        pub unsafe fn $name(x: ptr) -> bool {
            ((x as uptr) & $mask1 == $loc1) && ((inc_ptr::<ptr>(x, 1) as uptr) == $loc2)
        }
    };

    ($name:ident, $mask1:literal, $loc1:literal, $mask2:literal, $loc2:literal) => {
        #[inline(always)]
        pub unsafe fn $name(x: ptr) -> bool {
            ((x as uptr) & $mask1 == $loc1) && ((inc_ptr::<ptr>(x, 1) as uptr) & $mask2 == $loc2)
        }
    };
}

predicate!(Sfixnump, 0x7, 0x0);
predicate!(Scharp, 0xFF, 0x16);
predicate!(Snullp, 0x26);
predicate!(Seof_objectp, 0x36);
predicate!(Sbwp_objectp, 0x4E);
predicate!(Sbooleanp, 0xF7, 0x6);
predicate!(Spairp, 0x7, 0x1);
predicate!(Ssymbolp, 0x7, 0x3);
predicate!(Sprocedurep, 0x7, 0x5);
predicate!(Sflonump, 0x7, 0x2);
predicate!(Svectorp, 0x7, 0x7, 0x7, 0x0);
predicate!(Sfxvectorp, 0x7, 0x7, 0x7, 0x3);
predicate!(Sbytevectorp, 0x7, 0x7, 0x3, 0x1);
predicate!(Sstringp, 0x7, 0x7, 0x7, 0x2);
predicate!(Sbignump, 0x7, 0x7, 0x1F, 0x6);
predicate!(Sboxp, 0x7, 0x7, 0x7F, 0xE);
predicate!(Sinexactnump, 0x7, 0x7, 0x36);
predicate!(Sexactnump, 0x7, 0x7, 0x56);
predicate!(Sratnump, 0x7, 0x7, 0x16);
predicate!(Sinputportp, 0x7, 0x7, 0x1FF, 0x11E);
predicate!(Soutputportp, 0x7, 0x7, 0x2FF, 0x21E);
predicate!(Srecordp, 0x7, 0x7, 0x7, 0x7);

// #define Svector_length(x) ((iptr)((uptr)(*((iptr *)((uptr)(x)+1)))>>4))
// #define Svector_ref(x,i) (((ptr *)((uptr)(x)+9))[i])
// #define Sfxvector_length(x) ((iptr)((uptr)(*((iptr *)((uptr)(x)+1)))>>4))
// #define Sfxvector_ref(x,i) (((ptr *)((uptr)(x)+9))[i])
// #define Sbytevector_length(x) ((iptr)((uptr)(*((iptr *)((uptr)(x)+1)))>>3))
// #define Sbytevector_u8_ref(x,i) (((octet *)((uptr)(x)+9))[i])
/* Warning: Sbytevector_data(x) returns a pointer into x. */
// #define Sbytevector_data(x) &Sbytevector_u8_ref(x,0)

#[inline(always)]
pub unsafe fn Sstring_length(x: ptr) -> iptr {
    ((inc_ptr::<iptr>(x, 1) as uptr) >> 4) as iptr
}

// #define Sstring_ref(x,i) Schar_value(((string_char *)((uptr)(x)+9))[i])

#[inline(always)]
pub unsafe fn Sunbox(x: ptr) -> ptr {
    inc_ptr(x, 9)
}

#[inline(always)]
pub const fn Sfixnum(x: iptr) -> ptr {
    ((x * 8) as uptr) as ptr
}

#[inline(always)]
pub const fn Schar(x: string_char) -> ptr {
    ((x << 8 | 0x16) as uptr) as ptr
}

#[inline(always)]
pub const fn Sboolean(x: bool) -> ptr {
    if x {
        Strue
    } else {
        Sfalse
    }
}

#[inline(always)]
pub unsafe fn Scar(x: ptr) -> ptr {
    inc_ptr(x, 7)
}

#[inline(always)]
pub unsafe fn Scdr(x: ptr) -> ptr {
    inc_ptr(x, 15)
}

#[inline(always)]
pub unsafe fn Sflonum_value(x: ptr) -> raw::c_double {
    inc_ptr(x, 6)
}

#[inline(always)]
pub unsafe fn Sforeign_callable_entry_point(x: ptr) -> Option<unsafe extern "C" fn()> {
    inc_ptr(x, 65)
}

#[inline(always)]
pub unsafe fn Sforeign_callable_code_object(x: unsafe extern "C" fn()) -> ptr {
    ((x as uptr) - 65) as ptr
}
