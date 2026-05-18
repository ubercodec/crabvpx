use std::ffi::c_void;
pub const VPX_CS_SRGB: u32 = 7;
pub const VPX_CS_RESERVED: u32 = 6;
pub const VPX_CS_BT_2020: u32 = 5;
pub const VPX_CS_SMPTE_240: u32 = 4;
pub const VPX_CS_SMPTE_170: u32 = 3;
pub const VPX_CS_BT_709: u32 = 2;
pub const VPX_CS_BT_601: u32 = 1;
pub const VPX_CS_UNKNOWN: u32 = 0;
pub const VPX_CR_FULL_RANGE: u32 = 1;
pub const VPX_CR_STUDIO_RANGE: u32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Yv12BufferConfig {
    pub y_width: i32,
    pub y_height: i32,
    pub y_crop_width: i32,
    pub y_crop_height: i32,
    pub y_stride: i32,
    pub uv_width: i32,
    pub uv_height: i32,
    pub uv_crop_width: i32,
    pub uv_crop_height: i32,
    pub uv_stride: i32,
    pub alpha_width: i32,
    pub alpha_height: i32,
    pub alpha_stride: i32,
    pub y_buffer: *mut u8,
    pub u_buffer: *mut u8,
    pub v_buffer: *mut u8,
    pub alpha_buffer: *mut u8,
    pub buffer_alloc: *mut u8,
    pub buffer_alloc_sz: usize,
    pub border: i32,
    pub frame_size: usize,
    pub subsampling_x: i32,
    pub subsampling_y: i32,
    pub bit_depth: u32,
    pub color_space: u32,
    pub color_range: u32,
    pub render_width: i32,
    pub render_height: i32,
    pub corrupted: i32,
    pub flags: i32,
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_setup_intra_recon(mut ybf: *mut Yv12BufferConfig) {
    unsafe {
        let mut i: i32 = 0;
        core::ptr::write_bytes(
            (*ybf)
                .y_buffer
                .offset(-(1 as isize))
                .offset(-((*ybf).y_stride as isize)) as *mut c_void as *mut u8,
            127 as i32 as u8,
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
            127 as i32 as u8,
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
            127 as i32 as u8,
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
            127 as i32 as u8,
            ((*ybf).y_width + 5 as i32) as usize,
        );
        core::ptr::write_bytes(
            (*ybf)
                .u_buffer
                .offset(-(1 as isize))
                .offset(-((*ybf).uv_stride as isize)) as *mut c_void as *mut u8,
            127 as i32 as u8,
            ((*ybf).uv_width + 5 as i32) as usize,
        );
        core::ptr::write_bytes(
            (*ybf)
                .v_buffer
                .offset(-(1 as isize))
                .offset(-((*ybf).uv_stride as isize)) as *mut c_void as *mut u8,
            127 as i32 as u8,
            ((*ybf).uv_width + 5 as i32) as usize,
        );
    }
}
