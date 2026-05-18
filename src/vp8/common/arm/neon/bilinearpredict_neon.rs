use std::ffi::c_void;
use std::arch::aarch64::*;
extern "Rust" {
}
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
unsafe fn uint32_to_mem(mut buf: *mut u8, mut a: u32) {
    core::ptr::copy_nonoverlapping(&raw mut a as *const c_void as *const u8, buf as *mut c_void as *mut u8, 4 as size_t);
}
static mut bifilter4_coeff: [[u8; 2]; 8] = [
    [
        128 as u8,
        0 as u8,
    ],
    [
        112 as u8,
        16 as u8,
    ],
    [
        96 as u8,
        32 as u8,
    ],
    [
        80 as u8,
        48 as u8,
    ],
    [
        64 as u8,
        64 as u8,
    ],
    [
        48 as u8,
        80 as u8,
    ],
    [
        32 as u8,
        96 as u8,
    ],
    [
        16 as u8,
        112 as u8,
    ],
];
