use std::ffi::c_void;
unsafe extern "Rust" {
    pub type Sfilex;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sbuf {
    pub _base: *mut u8,
    pub _size: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sfile {
    pub _p: *mut u8,
    pub _r: i32,
    pub _w: i32,
    pub _flags: i16,
    pub _file: i16,
    pub _bf: Sbuf,
    pub _lbfsize: i32,
    pub _cookie: *mut c_void,
    pub _close: Option<unsafe fn(*mut c_void) -> i32>,
    pub _read: Option<unsafe fn(*mut c_void, *mut i8, i32) -> i32>,
    pub _seek: Option<unsafe fn(*mut c_void, i64, i32) -> i64>,
    pub _write: Option<unsafe fn(*mut c_void, *const i8, i32) -> i32>,
    pub _ub: Sbuf,
    pub _extra: *mut Sfilex,
    pub _ur: i32,
    pub _ubuf: [u8; 3],
    pub _nbuf: [u8; 1],
    pub _lb: Sbuf,
    pub _blksize: i32,
    pub _offset: i64,
}
pub type FILE = Sfile;
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
pub fn vpx_write_yuv_frame(_yuv_file: *mut FILE, _s: *mut Yv12BufferConfig) {}
