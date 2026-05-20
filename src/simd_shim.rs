#![cfg(not(target_arch = "aarch64"))]

use core::ffi::{c_uchar, c_int, c_short, c_void, c_char};
pub type ptrdiff_t = isize;

unsafe extern "C" {


    fn vp8_loop_filter_bh_c(y_ptr: *mut c_uchar, u_ptr: *mut c_uchar, v_ptr: *mut c_uchar, y_stride: c_int, uv_stride: c_int, lfi: *mut c_void);
    fn vp8_loop_filter_bv_c(y_ptr: *mut c_uchar, u_ptr: *mut c_uchar, v_ptr: *mut c_uchar, y_stride: c_int, uv_stride: c_int, lfi: *mut c_void);
    fn vp8_loop_filter_mbh_c(y_ptr: *mut c_uchar, u_ptr: *mut c_uchar, v_ptr: *mut c_uchar, y_stride: c_int, uv_stride: c_int, lfi: *mut c_void);
    fn vp8_loop_filter_mbv_c(y_ptr: *mut c_uchar, u_ptr: *mut c_uchar, v_ptr: *mut c_uchar, y_stride: c_int, uv_stride: c_int, lfi: *mut c_void);
    fn vp8_loop_filter_bhs_c(y_ptr: *mut c_uchar, y_stride: c_int, blimit: *const c_uchar);
    fn vp8_loop_filter_bvs_c(y_ptr: *mut c_uchar, y_stride: c_int, blimit: *const c_uchar);

    fn vp8_loop_filter_simple_horizontal_edge_c(y_ptr: *mut c_uchar, y_stride: c_int, blimit: *const c_uchar);
    fn vp8_loop_filter_simple_vertical_edge_c(y_ptr: *mut c_uchar, y_stride: c_int, blimit: *const c_uchar);



}









#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_bh_neon(y_ptr: *mut c_uchar, u_ptr: *mut c_uchar, v_ptr: *mut c_uchar, y_stride: c_int, uv_stride: c_int, lfi: *mut c_void) {
    vp8_loop_filter_bh_c(y_ptr, u_ptr, v_ptr, y_stride, uv_stride, lfi);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_bv_neon(y_ptr: *mut c_uchar, u_ptr: *mut c_uchar, v_ptr: *mut c_uchar, y_stride: c_int, uv_stride: c_int, lfi: *mut c_void) {
    vp8_loop_filter_bv_c(y_ptr, u_ptr, v_ptr, y_stride, uv_stride, lfi);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_mbh_neon(y_ptr: *mut c_uchar, u_ptr: *mut c_uchar, v_ptr: *mut c_uchar, y_stride: c_int, uv_stride: c_int, lfi: *mut c_void) {
    vp8_loop_filter_mbh_c(y_ptr, u_ptr, v_ptr, y_stride, uv_stride, lfi);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_mbv_neon(y_ptr: *mut c_uchar, u_ptr: *mut c_uchar, v_ptr: *mut c_uchar, y_stride: c_int, uv_stride: c_int, lfi: *mut c_void) {
    vp8_loop_filter_mbv_c(y_ptr, u_ptr, v_ptr, y_stride, uv_stride, lfi);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_bhs_neon(y_ptr: *mut c_uchar, y_stride: c_int, blimit: *const c_uchar) {
    vp8_loop_filter_bhs_c(y_ptr, y_stride, blimit);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_bvs_neon(y_ptr: *mut c_uchar, y_stride: c_int, blimit: *const c_uchar) {
    vp8_loop_filter_bvs_c(y_ptr, y_stride, blimit);
}



#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_mbhs_neon(y_ptr: *mut c_uchar, y_stride: c_int, blimit: *const c_uchar) {
    vp8_loop_filter_simple_horizontal_edge_c(y_ptr, y_stride, blimit);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_loop_filter_mbvs_neon(y_ptr: *mut c_uchar, y_stride: c_int, blimit: *const c_uchar) {
    vp8_loop_filter_simple_vertical_edge_c(y_ptr, y_stride, blimit);
}
















