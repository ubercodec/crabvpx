pub use crate::vp8::common::types::*;
pub type uint8_t = u8;

pub type __darwin_size_t = usize;
pub type vpx_color_space = ::core::ffi::c_uint;
pub const VPX_CS_SRGB: vpx_color_space = 7;
pub const VPX_CS_RESERVED: vpx_color_space = 6;
pub const VPX_CS_BT_2020: vpx_color_space = 5;
pub const VPX_CS_SMPTE_240: vpx_color_space = 4;
pub const VPX_CS_SMPTE_170: vpx_color_space = 3;
pub const VPX_CS_BT_709: vpx_color_space = 2;
pub const VPX_CS_BT_601: vpx_color_space = 1;
pub const VPX_CS_UNKNOWN: vpx_color_space = 0;
pub type vpx_color_space_t = vpx_color_space;
pub type vpx_color_range = ::core::ffi::c_uint;
pub const VPX_CR_FULL_RANGE: vpx_color_range = 1;
pub const VPX_CR_STUDIO_RANGE: vpx_color_range = 0;
pub type vpx_color_range_t = vpx_color_range;
pub type size_t = __darwin_size_t;

#[derive(Clone, Copy)]
#[repr(align(32))]
struct Align32([u8; 32]);

pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub fn vp8_yv12_de_alloc_frame_buffer_safe(ybf: &mut YV12_BUFFER_CONFIG) {
    if ybf.buffer_alloc_sz > 0 {
        let alloc_size = ybf.buffer_alloc_sz / 32;
        let ptr = ybf.buffer_alloc as *mut Align32;
        unsafe {
            let _vec = Vec::from_raw_parts(ptr, alloc_size, alloc_size);
        }
    }
    *ybf = YV12_BUFFER_CONFIG {
        y_width: 0,
        y_height: 0,
        y_crop_width: 0,
        y_crop_height: 0,
        y_stride: 0,
        uv_width: 0,
        uv_height: 0,
        uv_crop_width: 0,
        uv_crop_height: 0,
        uv_stride: 0,
        alpha_width: 0,
        alpha_height: 0,
        alpha_stride: 0,
        y_buffer: core::ptr::null_mut(),
        u_buffer: core::ptr::null_mut(),
        v_buffer: core::ptr::null_mut(),
        alpha_buffer: core::ptr::null_mut(),
        buffer_alloc: core::ptr::null_mut(),
        buffer_alloc_sz: 0,
        border: 0,
        frame_size: 0,
        subsampling_x: 0,
        subsampling_y: 0,
        bit_depth: 0,
        color_space: 0,
        color_range: 0,
        render_width: 0,
        render_height: 0,
        corrupted: 0,
        flags: 0,
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_yv12_de_alloc_frame_buffer(
    mut ybf: *mut YV12_BUFFER_CONFIG,
) -> ::core::ffi::c_int {
    if !ybf.is_null() {
        unsafe {
            vp8_yv12_de_alloc_frame_buffer_safe(&mut *ybf);
        }
    } else {
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}

pub fn vp8_yv12_realloc_frame_buffer_safe(
    ybf: &mut YV12_BUFFER_CONFIG,
    width: i32,
    height: i32,
    border: i32,
) -> Result<(), i32> {
    let aligned_width = (width + 15) & !15;
    let aligned_height = (height + 15) & !15;
    let y_stride = (aligned_width + 2 * border + 31) & !31;
    let yplane_size = (aligned_height + 2 * border) * y_stride;
    let uv_width = aligned_width >> 1;
    let uv_height = aligned_height >> 1;
    let uv_stride = y_stride >> 1;
    let uvplane_size = (uv_height + border) * uv_stride;
    let frame_size = (yplane_size + 2 * uvplane_size) as usize;

    if ybf.buffer_alloc.is_null() {
        let alloc_size = (frame_size + 31) / 32;
        let mut vec = vec![Align32([0; 32]); alloc_size];
        ybf.buffer_alloc = vec.as_mut_ptr() as *mut u8;
        core::mem::forget(vec);
        if ybf.buffer_alloc.is_null() {
            ybf.buffer_alloc_sz = 0;
            return Err(-1);
        }
        ybf.buffer_alloc_sz = alloc_size * 32;
    }

    if ybf.buffer_alloc_sz < frame_size {
        return Err(-1);
    }
    if border & 0x1f != 0 {
        return Err(-3);
    }

    ybf.y_crop_width = width;
    ybf.y_crop_height = height;
    ybf.y_width = aligned_width;
    ybf.y_height = aligned_height;
    ybf.y_stride = y_stride;
    ybf.uv_crop_width = (width + 1) / 2;
    ybf.uv_crop_height = (height + 1) / 2;
    ybf.uv_width = uv_width;
    ybf.uv_height = uv_height;
    ybf.uv_stride = uv_stride;
    ybf.alpha_width = 0;
    ybf.alpha_height = 0;
    ybf.alpha_stride = 0;
    ybf.border = border;
    ybf.frame_size = frame_size;

    unsafe {
        ybf.y_buffer = ybf
            .buffer_alloc
            .offset((border * y_stride) as isize)
            .offset(border as isize);
        ybf.u_buffer = ybf
            .buffer_alloc
            .offset(yplane_size as isize)
            .offset(((border / 2) * uv_stride) as isize)
            .offset((border / 2) as isize);
        ybf.v_buffer = ybf
            .buffer_alloc
            .offset(yplane_size as isize)
            .offset(uvplane_size as isize)
            .offset(((border / 2) * uv_stride) as isize)
            .offset((border / 2) as isize);
    }
    ybf.alpha_buffer = core::ptr::null_mut();
    ybf.corrupted = 0;

    Ok(())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_yv12_realloc_frame_buffer(
    mut ybf: *mut YV12_BUFFER_CONFIG,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut border: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !ybf.is_null() {
        unsafe {
            match vp8_yv12_realloc_frame_buffer_safe(&mut *ybf, width, height, border) {
                Ok(_) => 0,
                Err(e) => e,
            }
        }
    } else {
        -(2 as ::core::ffi::c_int)
    }
}

pub fn vp8_yv12_alloc_frame_buffer_safe(
    ybf: &mut YV12_BUFFER_CONFIG,
    width: i32,
    height: i32,
    border: i32,
) -> Result<(), i32> {
    vp8_yv12_de_alloc_frame_buffer_safe(ybf);
    vp8_yv12_realloc_frame_buffer_safe(ybf, width, height, border)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_yv12_alloc_frame_buffer(
    mut ybf: *mut YV12_BUFFER_CONFIG,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut border: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !ybf.is_null() {
        unsafe {
            match vp8_yv12_alloc_frame_buffer_safe(&mut *ybf, width, height, border) {
                Ok(_) => 0,
                Err(e) => e,
            }
        }
    } else {
        -(2 as ::core::ffi::c_int)
    }
}

