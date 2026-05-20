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
    let y_stride_u = y_stride as usize;
    let uv_stride_u = uv_stride as usize;

    assert!(y.len() >= 16 * y_stride_u, "Y buffer too small: len={}, req={}", y.len(), 16 * y_stride_u);
    let y_col = y_offset % y_stride_u;
    assert!(y_col + 16 <= y_stride_u, "Y column out of bounds: col={}, stride={}", y_col, y_stride_u);
    assert!(y_offset + 15 * y_stride_u + 16 <= y.len(), "Y offset out of bounds");

    assert!(u.len() >= 8 * uv_stride_u, "U buffer too small");
    let u_col = u_offset % uv_stride_u;
    assert!(u_col + 8 <= uv_stride_u, "U column out of bounds");
    assert!(u_offset + 7 * uv_stride_u + 8 <= u.len(), "U offset out of bounds");

    assert!(v.len() >= 8 * uv_stride_u, "V buffer too small");
    let v_col = v_offset % uv_stride_u;
    assert!(v_col + 8 <= uv_stride_u, "V column out of bounds");
    assert!(v_offset + 7 * uv_stride_u + 8 <= v.len(), "V offset out of bounds");

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
    let y_stride_u = y_stride as usize;
    let uv_stride_u = uv_stride as usize;

    assert!(y.len() >= 16 * y_stride_u, "Y buffer too small");
    let y_col = y_offset % y_stride_u;
    assert!(y_col + 16 <= y_stride_u, "Y column out of bounds");
    assert!(y_offset + 15 * y_stride_u + 16 <= y.len(), "Y offset out of bounds");

    assert!(u.len() >= 8 * uv_stride_u, "U buffer too small");
    let u_col = u_offset % uv_stride_u;
    assert!(u_col + 8 <= uv_stride_u, "U column out of bounds");
    assert!(u_offset + 7 * uv_stride_u + 8 <= u.len(), "U offset out of bounds");

    assert!(v.len() >= 8 * uv_stride_u, "V buffer too small");
    let v_col = v_offset % uv_stride_u;
    assert!(v_col + 8 <= uv_stride_u, "V column out of bounds");
    assert!(v_offset + 7 * uv_stride_u + 8 <= v.len(), "V offset out of bounds");

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
    let y_stride_u = y_stride as usize;
    let uv_stride_u = uv_stride as usize;

    assert!(y_offset >= 4 * y_stride_u, "Y offset too small for above row access");
    let y_col = y_offset % y_stride_u;
    assert!(y_col + 16 <= y_stride_u, "Y column out of bounds");
    assert!(y_offset + 3 * y_stride_u + y_col + 16 <= y.len(), "Y buffer too small for below row access");

    assert!(u_offset >= 4 * uv_stride_u, "U offset too small");
    let u_col = u_offset % uv_stride_u;
    assert!(u_col + 8 <= uv_stride_u, "U column out of bounds");
    assert!(u_offset + 3 * uv_stride_u + u_col + 8 <= u.len(), "U buffer too small");

    assert!(v_offset >= 4 * uv_stride_u, "V offset too small");
    let v_col = v_offset % uv_stride_u;
    assert!(v_col + 8 <= uv_stride_u, "V column out of bounds");
    assert!(v_offset + 3 * uv_stride_u + v_col + 8 <= v.len(), "V buffer too small");

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
    let y_stride_u = y_stride as usize;
    let uv_stride_u = uv_stride as usize;

    let y_col = y_offset % y_stride_u;
    assert!(y_col >= 4, "Y offset too small for left MB access");
    assert!(y_col + 4 <= y_stride_u, "Y column out of bounds");
    assert!(y.len() >= 16 * y_stride_u, "Y buffer too small");
    assert!(y_offset + 15 * y_stride_u + 4 <= y.len(), "Y offset out of bounds");

    let u_col = u_offset % uv_stride_u;
    assert!(u_col >= 4, "U offset too small");
    assert!(u_col + 4 <= uv_stride_u, "U column out of bounds");
    assert!(u.len() >= 8 * uv_stride_u, "U buffer too small");
    assert!(u_offset + 7 * uv_stride_u + 4 <= u.len(), "U offset out of bounds");

    let v_col = v_offset % uv_stride_u;
    assert!(v_col >= 4, "V offset too small");
    assert!(v_col + 4 <= uv_stride_u, "V column out of bounds");
    assert!(v.len() >= 8 * uv_stride_u, "V buffer too small");
    assert!(v_offset + 7 * uv_stride_u + 4 <= v.len(), "V offset out of bounds");

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
    let y_stride_u = y_stride as usize;
    assert!(y.len() >= 16 * y_stride_u, "Y buffer too small");
    let y_col = y_offset % y_stride_u;
    assert!(y_col + 16 <= y_stride_u, "Y column out of bounds");
    assert!(y_offset + 15 * y_stride_u + 16 <= y.len(), "Y offset out of bounds");
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
    let y_stride_u = y_stride as usize;
    assert!(y.len() >= 16 * y_stride_u, "Y buffer too small");
    let y_col = y_offset % y_stride_u;
    assert!(y_col + 16 <= y_stride_u, "Y column out of bounds");
    assert!(y_offset + 15 * y_stride_u + 16 <= y.len(), "Y offset out of bounds");
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
    let y_stride_u = y_stride as usize;
    assert!(y_offset >= 2 * y_stride_u, "Y offset too small for above row access");
    let y_col = y_offset % y_stride_u;
    assert!(y_col + 16 <= y_stride_u, "Y column out of bounds");
    assert!(y_offset + y_stride_u + y_col + 16 <= y.len(), "Y buffer too small for below row access");
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
    let y_stride_u = y_stride as usize;
    let y_col = y_offset % y_stride_u;
    assert!(y_col >= 2, "Y offset too small for left MB access");
    assert!(y_col + 2 <= y_stride_u, "Y column out of bounds");
    assert!(y.len() >= 16 * y_stride_u, "Y buffer too small");
    assert!(y_offset + 15 * y_stride_u + 2 <= y.len(), "Y offset out of bounds");
    assert!(!blimit.is_empty());

    unsafe {
        vp8_loop_filter_mbvs_neon(
            y.as_mut_ptr().add(y_offset),
            y_stride,
            blimit.as_ptr(),
        );
    }
}
