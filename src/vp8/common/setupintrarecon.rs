pub use crate::vpx::src::vpx_image::{
    VPX_CR_FULL_RANGE, VPX_CR_STUDIO_RANGE, VPX_CS_BT_601, VPX_CS_BT_709, VPX_CS_BT_2020,
    VPX_CS_RESERVED, VPX_CS_SMPTE_170, VPX_CS_SMPTE_240, VPX_CS_SRGB, VPX_CS_UNKNOWN,
};
use crate::vpx_scale::generic::yv12config::Yv12BufferConfig;
use std::ffi::c_void;

#[unsafe(no_mangle)]
pub unsafe fn vp8_setup_intra_recon(mut ybf: *mut Yv12BufferConfig) {
    unsafe {
        let mut i: i32 = 0;
        core::ptr::write_bytes(
            (*ybf)
                .y_buffer
                .offset(-(1 as isize))
                .offset(-((*ybf).y_stride as isize)) as *mut c_void as *mut u8,
            127 as u8,
            ((*ybf).y_width + 5 as i32) as usize,
        );
        i = 0 as i32;
        while i < (*ybf).y_height {
            *(*ybf)
                .y_buffer
                .offset(((*ybf).y_stride * i - 1 as i32) as isize) = 129 as u8;
            i += 1;
        }
        core::ptr::write_bytes(
            (*ybf)
                .u_buffer
                .offset(-(1 as isize))
                .offset(-((*ybf).uv_stride as isize)) as *mut c_void as *mut u8,
            127 as u8,
            ((*ybf).uv_width + 5 as i32) as usize,
        );
        i = 0 as i32;
        while i < (*ybf).uv_height {
            *(*ybf)
                .u_buffer
                .offset(((*ybf).uv_stride * i - 1 as i32) as isize) = 129 as u8;
            i += 1;
        }
        core::ptr::write_bytes(
            (*ybf)
                .v_buffer
                .offset(-(1 as isize))
                .offset(-((*ybf).uv_stride as isize)) as *mut c_void as *mut u8,
            127 as u8,
            ((*ybf).uv_width + 5 as i32) as usize,
        );
        i = 0 as i32;
        while i < (*ybf).uv_height {
            *(*ybf)
                .v_buffer
                .offset(((*ybf).uv_stride * i - 1 as i32) as isize) = 129 as u8;
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_setup_intra_recon_top_line(mut ybf: *mut Yv12BufferConfig) {
    unsafe {
        core::ptr::write_bytes(
            (*ybf)
                .y_buffer
                .offset(-(1 as isize))
                .offset(-((*ybf).y_stride as isize)) as *mut c_void as *mut u8,
            127 as u8,
            ((*ybf).y_width + 5 as i32) as usize,
        );
        core::ptr::write_bytes(
            (*ybf)
                .u_buffer
                .offset(-(1 as isize))
                .offset(-((*ybf).uv_stride as isize)) as *mut c_void as *mut u8,
            127 as u8,
            ((*ybf).uv_width + 5 as i32) as usize,
        );
        core::ptr::write_bytes(
            (*ybf)
                .v_buffer
                .offset(-(1 as isize))
                .offset(-((*ybf).uv_stride as isize)) as *mut c_void as *mut u8,
            127 as u8,
            ((*ybf).uv_width + 5 as i32) as usize,
        );
    }
}
