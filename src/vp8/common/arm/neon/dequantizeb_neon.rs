use std::arch::aarch64::*;
pub type int8_t = i8;
pub type int16_t = i16;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct int16x8x2_t {
    pub val: [int16x8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockd {
    pub qcoeff: *mut ::core::ffi::c_short,
    pub dqcoeff: *mut ::core::ffi::c_short,
    pub predictor: *mut ::core::ffi::c_uchar,
    pub dequant: *mut ::core::ffi::c_short,
    pub offset: ::core::ffi::c_int,
    pub eob: *mut ::core::ffi::c_char,
    pub bmi: b_mode_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union b_mode_info {
    pub as_mode: B_PREDICTION_MODE,
    pub mv: int_mv,
}
pub use crate::vp8::common::types::*;

pub type BLOCKD = blockd;
