use std::arch::aarch64::*;
extern "Rust" {
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type __darwin_ptrdiff_t = isize;
pub type __darwin_size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uint32x2x2_t {
    pub val: [uint32x2_t; 2],
}
pub type size_t = __darwin_size_t;
pub type ptrdiff_t = __darwin_ptrdiff_t;
#[inline]
unsafe fn uint32_to_mem(mut buf: *mut uint8_t, mut a: uint32_t) {
    memcpy(
        buf as *mut ::core::ffi::c_void,
        &raw mut a as *const ::core::ffi::c_void,
        4 as size_t,
    );
}
static mut vp8_sub_pel_filters: [[int8_t; 8]; 8] = [
    [
        0 as i32 as int8_t,
        0 as i32 as int8_t,
        -(128 as i32) as int8_t,
        0 as i32 as int8_t,
        0 as i32 as int8_t,
        0 as i32 as int8_t,
        0 as i32 as int8_t,
        0 as i32 as int8_t,
    ],
    [
        0 as i32 as int8_t,
        -(6 as i32) as int8_t,
        123 as i32 as int8_t,
        12 as i32 as int8_t,
        -(1 as i32) as int8_t,
        0 as i32 as int8_t,
        0 as i32 as int8_t,
        0 as i32 as int8_t,
    ],
    [
        2 as i32 as int8_t,
        -(11 as i32) as int8_t,
        108 as i32 as int8_t,
        36 as i32 as int8_t,
        -(8 as i32) as int8_t,
        1 as i32 as int8_t,
        0 as i32 as int8_t,
        0 as i32 as int8_t,
    ],
    [
        0 as i32 as int8_t,
        -(9 as i32) as int8_t,
        93 as i32 as int8_t,
        50 as i32 as int8_t,
        -(6 as i32) as int8_t,
        0 as i32 as int8_t,
        0 as i32 as int8_t,
        0 as i32 as int8_t,
    ],
    [
        3 as i32 as int8_t,
        -(16 as i32) as int8_t,
        77 as i32 as int8_t,
        77 as i32 as int8_t,
        -(16 as i32) as int8_t,
        3 as i32 as int8_t,
        0 as i32 as int8_t,
        0 as i32 as int8_t,
    ],
    [
        0 as i32 as int8_t,
        -(6 as i32) as int8_t,
        50 as i32 as int8_t,
        93 as i32 as int8_t,
        -(9 as i32) as int8_t,
        0 as i32 as int8_t,
        0 as i32 as int8_t,
        0 as i32 as int8_t,
    ],
    [
        1 as i32 as int8_t,
        -(8 as i32) as int8_t,
        36 as i32 as int8_t,
        108 as i32 as int8_t,
        -(11 as i32) as int8_t,
        2 as i32 as int8_t,
        0 as i32 as int8_t,
        0 as i32 as int8_t,
    ],
    [
        0 as i32 as int8_t,
        -(1 as i32) as int8_t,
        12 as i32 as int8_t,
        123 as i32 as int8_t,
        -(6 as i32) as int8_t,
        0 as i32 as int8_t,
        0 as i32 as int8_t,
        0 as i32 as int8_t,
    ],
];
static mut abs_filters: [[uint8_t; 8]; 8] = [
    [
        0 as i32 as uint8_t,
        0 as i32 as uint8_t,
        128 as i32 as uint8_t,
        0 as i32 as uint8_t,
        0 as i32 as uint8_t,
        0 as i32 as uint8_t,
        0 as i32 as uint8_t,
        0 as i32 as uint8_t,
    ],
    [
        0 as i32 as uint8_t,
        6 as i32 as uint8_t,
        123 as i32 as uint8_t,
        12 as i32 as uint8_t,
        1 as i32 as uint8_t,
        0 as i32 as uint8_t,
        0 as i32 as uint8_t,
        0 as i32 as uint8_t,
    ],
    [
        2 as i32 as uint8_t,
        11 as i32 as uint8_t,
        108 as i32 as uint8_t,
        36 as i32 as uint8_t,
        8 as i32 as uint8_t,
        1 as i32 as uint8_t,
        0 as i32 as uint8_t,
        0 as i32 as uint8_t,
    ],
    [
        0 as i32 as uint8_t,
        9 as i32 as uint8_t,
        93 as i32 as uint8_t,
        50 as i32 as uint8_t,
        6 as i32 as uint8_t,
        0 as i32 as uint8_t,
        0 as i32 as uint8_t,
        0 as i32 as uint8_t,
    ],
    [
        3 as i32 as uint8_t,
        16 as i32 as uint8_t,
        77 as i32 as uint8_t,
        77 as i32 as uint8_t,
        16 as i32 as uint8_t,
        3 as i32 as uint8_t,
        0 as i32 as uint8_t,
        0 as i32 as uint8_t,
    ],
    [
        0 as i32 as uint8_t,
        6 as i32 as uint8_t,
        50 as i32 as uint8_t,
        93 as i32 as uint8_t,
        9 as i32 as uint8_t,
        0 as i32 as uint8_t,
        0 as i32 as uint8_t,
        0 as i32 as uint8_t,
    ],
    [
        1 as i32 as uint8_t,
        8 as i32 as uint8_t,
        36 as i32 as uint8_t,
        108 as i32 as uint8_t,
        11 as i32 as uint8_t,
        2 as i32 as uint8_t,
        0 as i32 as uint8_t,
        0 as i32 as uint8_t,
    ],
    [
        0 as i32 as uint8_t,
        1 as i32 as uint8_t,
        12 as i32 as uint8_t,
        123 as i32 as uint8_t,
        6 as i32 as uint8_t,
        0 as i32 as uint8_t,
        0 as i32 as uint8_t,
        0 as i32 as uint8_t,
    ],
];
