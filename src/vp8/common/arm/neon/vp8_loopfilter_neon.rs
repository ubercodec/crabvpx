use std::arch::aarch64::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uint8x16x2_t {
    pub val: [uint8x16_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uint16x8x2_t {
    pub val: [uint16x8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uint32x4x2_t {
    pub val: [uint32x4_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uint8x8x4_t {
    pub val: [uint8x8_t; 4],
}
pub type loopfilter_y_neon = unsafe fn(
    *mut u8,
    i32,
    u8,
    u8,
    u8,
) -> ();
pub type loopfilter_uv_neon = unsafe fn(
    *mut u8,
    i32,
    u8,
    u8,
    u8,
    *mut u8,
) -> ();
