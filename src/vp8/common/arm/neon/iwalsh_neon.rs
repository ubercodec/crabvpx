use std::arch::aarch64::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct int16x4x2_t {
    pub val: [int16x4_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct int32x2x2_t {
    pub val: [int32x2_t; 2],
}
