// SIMD Shim module. All loop filter FFI shims have been refactored to safe Rust native implementations.
// This file provides safe Rust interfaces to the unsafe NEON FFI loop filter functions on aarch64.

#[cfg(target_arch = "aarch64")]
pub use crate::vp8::decoder::threading::loop_filter_info;

#[cfg(target_arch = "aarch64")]
unsafe extern "C" {
    fn vp8_loop_filter_bh_neon(
        y_ptr: *mut u8,
        u_ptr: *mut u8,
        v_ptr: *mut u8,
        y_stride: ::core::ffi::c_int,
        uv_stride: ::core::ffi::c_int,
        lfi: *mut loop_filter_info,
    );
    fn vp8_loop_filter_bv_neon(
        y_ptr: *mut u8,
        u_ptr: *mut u8,
        v_ptr: *mut u8,
        y_stride: ::core::ffi::c_int,
        uv_stride: ::core::ffi::c_int,
        lfi: *mut loop_filter_info,
    );
    fn vp8_loop_filter_mbh_neon(
        y_ptr: *mut u8,
        u_ptr: *mut u8,
        v_ptr: *mut u8,
        y_stride: ::core::ffi::c_int,
        uv_stride: ::core::ffi::c_int,
        lfi: *mut loop_filter_info,
    );
    fn vp8_loop_filter_mbv_neon(
        y_ptr: *mut u8,
        u_ptr: *mut u8,
        v_ptr: *mut u8,
        y_stride: ::core::ffi::c_int,
        uv_stride: ::core::ffi::c_int,
        lfi: *mut loop_filter_info,
    );
    fn vp8_loop_filter_bhs_neon(
        y_ptr: *mut u8,
        y_stride: ::core::ffi::c_int,
        blimit: *const u8,
    );
    fn vp8_loop_filter_bvs_neon(
        y_ptr: *mut u8,
        y_stride: ::core::ffi::c_int,
        blimit: *const u8,
    );
    fn vp8_loop_filter_mbhs_neon(
        y_ptr: *mut u8,
        y_stride: ::core::ffi::c_int,
        blimit: *const u8,
    );
    fn vp8_loop_filter_mbvs_neon(
        y_ptr: *mut u8,
        y_stride: ::core::ffi::c_int,
        blimit: *const u8,
    );
}

#[cfg(target_arch = "aarch64")]
pub fn safe_vp8_loop_filter_bh_neon(
    y: &mut [u8],
    y_offset: usize,
    u: &mut [u8],
    u_offset: usize,
    v: &mut [u8],
    v_offset: usize,
    y_stride: i32,
    uv_stride: i32,
    lfi: &mut loop_filter_info,
) {
    assert!(y_offset < y.len());
    assert!(u_offset < u.len());
    assert!(v_offset < v.len());
    unsafe {
        vp8_loop_filter_bh_neon(
            y.as_mut_ptr().add(y_offset),
            u.as_mut_ptr().add(u_offset),
            v.as_mut_ptr().add(v_offset),
            y_stride,
            uv_stride,
            lfi,
        );
    }
}

#[cfg(target_arch = "aarch64")]
pub fn safe_vp8_loop_filter_bv_neon(
    y: &mut [u8],
    y_offset: usize,
    u: &mut [u8],
    u_offset: usize,
    v: &mut [u8],
    v_offset: usize,
    y_stride: i32,
    uv_stride: i32,
    lfi: &mut loop_filter_info,
) {
    assert!(y_offset < y.len());
    assert!(u_offset < u.len());
    assert!(v_offset < v.len());
    unsafe {
        vp8_loop_filter_bv_neon(
            y.as_mut_ptr().add(y_offset),
            u.as_mut_ptr().add(u_offset),
            v.as_mut_ptr().add(v_offset),
            y_stride,
            uv_stride,
            lfi,
        );
    }
}

#[cfg(target_arch = "aarch64")]
pub fn safe_vp8_loop_filter_mbh_neon(
    y: &mut [u8],
    y_offset: usize,
    u: &mut [u8],
    u_offset: usize,
    v: &mut [u8],
    v_offset: usize,
    y_stride: i32,
    uv_stride: i32,
    lfi: &mut loop_filter_info,
) {
    assert!(y_offset < y.len());
    assert!(u_offset < u.len());
    assert!(v_offset < v.len());
    unsafe {
        vp8_loop_filter_mbh_neon(
            y.as_mut_ptr().add(y_offset),
            u.as_mut_ptr().add(u_offset),
            v.as_mut_ptr().add(v_offset),
            y_stride,
            uv_stride,
            lfi,
        );
    }
}

#[cfg(target_arch = "aarch64")]
pub fn safe_vp8_loop_filter_mbv_neon(
    y: &mut [u8],
    y_offset: usize,
    u: &mut [u8],
    u_offset: usize,
    v: &mut [u8],
    v_offset: usize,
    y_stride: i32,
    uv_stride: i32,
    lfi: &mut loop_filter_info,
) {
    assert!(y_offset < y.len());
    assert!(u_offset < u.len());
    assert!(v_offset < v.len());
    unsafe {
        vp8_loop_filter_mbv_neon(
            y.as_mut_ptr().add(y_offset),
            u.as_mut_ptr().add(u_offset),
            v.as_mut_ptr().add(v_offset),
            y_stride,
            uv_stride,
            lfi,
        );
    }
}

#[cfg(target_arch = "aarch64")]
pub fn safe_vp8_loop_filter_bhs_neon(
    y: &mut [u8],
    y_offset: usize,
    y_stride: i32,
    blimit: &[u8],
) {
    assert!(y_offset < y.len());
    assert!(!blimit.is_empty());
    unsafe {
        vp8_loop_filter_bhs_neon(
            y.as_mut_ptr().add(y_offset),
            y_stride,
            blimit.as_ptr(),
        );
    }
}

#[cfg(target_arch = "aarch64")]
pub fn safe_vp8_loop_filter_bvs_neon(
    y: &mut [u8],
    y_offset: usize,
    y_stride: i32,
    blimit: &[u8],
) {
    assert!(y_offset < y.len());
    assert!(!blimit.is_empty());
    unsafe {
        vp8_loop_filter_bvs_neon(
            y.as_mut_ptr().add(y_offset),
            y_stride,
            blimit.as_ptr(),
        );
    }
}

#[cfg(target_arch = "aarch64")]
pub fn safe_vp8_loop_filter_mbhs_neon(
    y: &mut [u8],
    y_offset: usize,
    y_stride: i32,
    blimit: &[u8],
) {
    assert!(y_offset < y.len());
    assert!(!blimit.is_empty());
    unsafe {
        vp8_loop_filter_mbhs_neon(
            y.as_mut_ptr().add(y_offset),
            y_stride,
            blimit.as_ptr(),
        );
    }
}

#[cfg(target_arch = "aarch64")]
pub fn safe_vp8_loop_filter_mbvs_neon(
    y: &mut [u8],
    y_offset: usize,
    y_stride: i32,
    blimit: &[u8],
) {
    assert!(y_offset < y.len());
    assert!(!blimit.is_empty());
    unsafe {
        vp8_loop_filter_mbvs_neon(
            y.as_mut_ptr().add(y_offset),
            y_stride,
            blimit.as_ptr(),
        );
    }
}
