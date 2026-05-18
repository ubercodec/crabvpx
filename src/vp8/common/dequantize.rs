unsafe extern "C" {
    fn vp8_short_idct4x4llm_c(
        input: *mut ::core::ffi::c_short,
        pred_ptr: *mut ::core::ffi::c_uchar,
        pred_stride: ::core::ffi::c_int,
        dst_ptr: *mut ::core::ffi::c_uchar,
        dst_stride: ::core::ffi::c_int,
    );
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub use crate::vp8::common::types::*;
pub type uint32_t = u32;

pub type size_t = __darwin_size_t;
pub type __darwin_size_t = usize;
pub fn vp8_dequantize_b_c(
    d: &mut BLOCKD,
    DQC: &[i16],
) {
    assert!(!d.dqcoeff.is_null(), "dqcoeff is null");
    assert!(!d.qcoeff.is_null(), "qcoeff is null");
    let dq = unsafe { std::slice::from_raw_parts_mut(d.dqcoeff, 16) };
    let q = unsafe { std::slice::from_raw_parts(d.qcoeff, 16) };
    for i in 0..16 {
        dq[i] = (q[i] as i32 * DQC[i] as i32) as i16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_dequant_idct_add_c(
    mut input: *mut ::core::ffi::c_short,
    mut dq: *mut ::core::ffi::c_short,
    mut dest: *mut ::core::ffi::c_uchar,
    mut stride: ::core::ffi::c_int,
) { unsafe {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        *input.offset(i as isize) = (*dq.offset(i as isize) as ::core::ffi::c_int
            * *input.offset(i as isize) as ::core::ffi::c_int)
            as ::core::ffi::c_short;
        i += 1;
    }
    vp8_short_idct4x4llm_c(input, dest, stride, dest, stride);
    memset(
        input as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        32 as size_t,
    );
}}
