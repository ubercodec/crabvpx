unsafe extern "Rust" {
    static vp8_norm: [::core::ffi::c_uchar; 256];
    fn vp8dx_bool_decoder_fill(br: *mut BOOL_DECODER);
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: i32,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
}
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
pub type __darwin_natural_t = ::core::ffi::c_uint;
pub type __darwin_size_t = usize;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: Option<unsafe fn(*mut ::core::ffi::c_void) -> ()>,
    pub __arg: *mut ::core::ffi::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_t {
    pub __sig: ::core::ffi::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [::core::ffi::c_char; 8176],
}
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type int16_t = i16;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type vpx_codec_err_t = ::core::ffi::c_uint;
pub const VPX_CODEC_LIST_END: vpx_codec_err_t = 9;
pub const VPX_CODEC_INVALID_PARAM: vpx_codec_err_t = 8;
pub const VPX_CODEC_CORRUPT_FRAME: vpx_codec_err_t = 7;
pub const VPX_CODEC_UNSUP_FEATURE: vpx_codec_err_t = 6;
pub const VPX_CODEC_UNSUP_BITSTREAM: vpx_codec_err_t = 5;
pub const VPX_CODEC_INCAPABLE: vpx_codec_err_t = 4;
pub const VPX_CODEC_ABI_MISMATCH: vpx_codec_err_t = 3;
pub const VPX_CODEC_MEM_ERROR: vpx_codec_err_t = 2;
pub const VPX_CODEC_ERROR: vpx_codec_err_t = 1;
pub const VPX_CODEC_OK: vpx_codec_err_t = 0;
pub type jmp_buf = [i32; 48];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_internal_error_info {
    pub error_code: vpx_codec_err_t,
    pub has_detail: i32,
    pub detail: [::core::ffi::c_char; 80],
    pub setjmp: i32,
    pub jmp: jmp_buf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yv12_buffer_config {
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
    pub y_buffer: *mut uint8_t,
    pub u_buffer: *mut uint8_t,
    pub v_buffer: *mut uint8_t,
    pub alpha_buffer: *mut uint8_t,
    pub buffer_alloc: *mut uint8_t,
    pub buffer_alloc_sz: size_t,
    pub border: i32,
    pub frame_size: size_t,
    pub subsampling_x: i32,
    pub subsampling_y: i32,
    pub bit_depth: ::core::ffi::c_uint,
    pub color_space: vpx_color_space_t,
    pub color_range: vpx_color_range_t,
    pub render_width: i32,
    pub render_height: i32,
    pub corrupted: i32,
    pub flags: i32,
}
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MV {
    pub row: ::core::ffi::c_short,
    pub col: ::core::ffi::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union int_mv {
    pub as_int: uint32_t,
    pub as_mv: MV,
}
pub type vp8_prob = ::core::ffi::c_uchar;
pub type ENTROPY_CONTEXT = ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ENTROPY_CONTEXT_PLANES {
    pub y1: [ENTROPY_CONTEXT; 4],
    pub u: [ENTROPY_CONTEXT; 2],
    pub v: [ENTROPY_CONTEXT; 2],
    pub y2: ENTROPY_CONTEXT,
}
pub type FRAME_TYPE = ::core::ffi::c_uint;
pub const INTER_FRAME: FRAME_TYPE = 1;
pub const KEY_FRAME: FRAME_TYPE = 0;
pub type B_PREDICTION_MODE = ::core::ffi::c_uint;
pub const B_MODE_COUNT: B_PREDICTION_MODE = 14;
pub const NEW4X4: B_PREDICTION_MODE = 13;
pub const ZERO4X4: B_PREDICTION_MODE = 12;
pub const ABOVE4X4: B_PREDICTION_MODE = 11;
pub const LEFT4X4: B_PREDICTION_MODE = 10;
pub const B_HU_PRED: B_PREDICTION_MODE = 9;
pub const B_HD_PRED: B_PREDICTION_MODE = 8;
pub const B_VL_PRED: B_PREDICTION_MODE = 7;
pub const B_VR_PRED: B_PREDICTION_MODE = 6;
pub const B_RD_PRED: B_PREDICTION_MODE = 5;
pub const B_LD_PRED: B_PREDICTION_MODE = 4;
pub const B_HE_PRED: B_PREDICTION_MODE = 3;
pub const B_VE_PRED: B_PREDICTION_MODE = 2;
pub const B_TM_PRED: B_PREDICTION_MODE = 1;
pub const B_DC_PRED: B_PREDICTION_MODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union b_mode_info {
    pub as_mode: B_PREDICTION_MODE,
    pub mv: int_mv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MB_MODE_INFO {
    pub mode: uint8_t,
    pub uv_mode: uint8_t,
    pub ref_frame: uint8_t,
    pub is_4x4: uint8_t,
    pub mv: int_mv,
    pub partitioning: uint8_t,
    pub mb_skip_coeff: uint8_t,
    pub need_to_clamp_mvs: uint8_t,
    pub segment_id: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct modeinfo {
    pub mbmi: MB_MODE_INFO,
    pub bmi: [b_mode_info; 16],
}
pub type MODE_INFO = modeinfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockd {
    pub qcoeff: *mut ::core::ffi::c_short,
    pub dqcoeff: *mut ::core::ffi::c_short,
    pub predictor: *mut ::core::ffi::c_uchar,
    pub dequant: *mut ::core::ffi::c_short,
    pub offset: i32,
    pub eob: *mut ::core::ffi::c_char,
    pub bmi: b_mode_info,
}
pub type BLOCKD = blockd;
pub type vp8_subpix_fn_t = Option<unsafe fn(
        *mut ::core::ffi::c_uchar,
        i32,
        i32,
        i32,
        *mut ::core::ffi::c_uchar,
        i32,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct macroblockd {
    pub predictor: [::core::ffi::c_uchar; 384],
    pub qcoeff: [::core::ffi::c_short; 400],
    pub dqcoeff: [::core::ffi::c_short; 400],
    pub eobs: [::core::ffi::c_char; 25],
    pub dequant_y1: [::core::ffi::c_short; 16],
    pub dequant_y1_dc: [::core::ffi::c_short; 16],
    pub dequant_y2: [::core::ffi::c_short; 16],
    pub dequant_uv: [::core::ffi::c_short; 16],
    pub block: [BLOCKD; 25],
    pub fullpixel_mask: i32,
    pub pre: YV12_BUFFER_CONFIG,
    pub dst: YV12_BUFFER_CONFIG,
    pub mode_info_context: *mut MODE_INFO,
    pub mode_info_stride: i32,
    pub frame_type: FRAME_TYPE,
    pub up_available: i32,
    pub left_available: i32,
    pub recon_above: [*mut ::core::ffi::c_uchar; 3],
    pub recon_left: [*mut ::core::ffi::c_uchar; 3],
    pub recon_left_stride: [i32; 2],
    pub above_context: *mut ENTROPY_CONTEXT_PLANES,
    pub left_context: *mut ENTROPY_CONTEXT_PLANES,
    pub segmentation_enabled: ::core::ffi::c_uchar,
    pub update_mb_segmentation_map: ::core::ffi::c_uchar,
    pub update_mb_segmentation_data: ::core::ffi::c_uchar,
    pub mb_segment_abs_delta: ::core::ffi::c_uchar,
    pub mb_segment_tree_probs: [vp8_prob; 3],
    pub segment_feature_data: [[::core::ffi::c_schar; 4]; 2],
    pub mode_ref_lf_delta_enabled: ::core::ffi::c_uchar,
    pub mode_ref_lf_delta_update: ::core::ffi::c_uchar,
    pub last_ref_lf_deltas: [::core::ffi::c_schar; 4],
    pub ref_lf_deltas: [::core::ffi::c_schar; 4],
    pub last_mode_lf_deltas: [::core::ffi::c_schar; 4],
    pub mode_lf_deltas: [::core::ffi::c_schar; 4],
    pub mb_to_left_edge: i32,
    pub mb_to_right_edge: i32,
    pub mb_to_top_edge: i32,
    pub mb_to_bottom_edge: i32,
    pub subpixel_predict: vp8_subpix_fn_t,
    pub subpixel_predict8x4: vp8_subpix_fn_t,
    pub subpixel_predict8x8: vp8_subpix_fn_t,
    pub subpixel_predict16x16: vp8_subpix_fn_t,
    pub current_bc: *mut ::core::ffi::c_void,
    pub corrupted: i32,
    pub error_info: vpx_internal_error_info,
}
pub type MACROBLOCKD = macroblockd;
pub type pthread_t = *mut ::core::ffi::c_void;
pub type mach_port_t = __darwin_mach_port_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8D_COMP {
    pub mb: MACROBLOCKD,
    pub dec_fb_ref: [*mut YV12_BUFFER_CONFIG; 4],
    pub common: VP8_COMMON,
    pub mbc: [vp8_reader; 9],
    pub oxcf: VP8D_CONFIG,
    pub fragments: FRAGMENT_DATA,
    pub b_multithreaded_rd: vpx_atomic_int,
    pub max_threads: i32,
    pub current_mb_col_main: i32,
    pub decoding_thread_count: ::core::ffi::c_uint,
    pub allocated_decoding_thread_count: i32,
    pub mt_baseline_filter_level: [i32; 4],
    pub sync_range: i32,
    pub mt_current_mb_col: *mut vpx_atomic_int,
    pub mt_yabove_row: *mut *mut ::core::ffi::c_uchar,
    pub mt_uabove_row: *mut *mut ::core::ffi::c_uchar,
    pub mt_vabove_row: *mut *mut ::core::ffi::c_uchar,
    pub mt_yleft_col: *mut *mut ::core::ffi::c_uchar,
    pub mt_uleft_col: *mut *mut ::core::ffi::c_uchar,
    pub mt_vleft_col: *mut *mut ::core::ffi::c_uchar,
    pub mb_row_di: *mut MB_ROW_DEC,
    pub de_thread_data: *mut DECODETHREAD_DATA,
    pub h_decoding_thread: *mut pthread_t,
    pub h_event_start_decoding: *mut semaphore_t,
    pub h_event_end_decoding: semaphore_t,
    pub ready_for_new_data: i32,
    pub prob_intra: vp8_prob,
    pub prob_last: vp8_prob,
    pub prob_gf: vp8_prob,
    pub prob_skip_false: vp8_prob,
    pub ec_enabled: i32,
    pub ec_active: i32,
    pub decoded_key_frame: i32,
    pub independent_partitions: i32,
    pub frame_corrupt_residual: i32,
    pub decrypt_cb: vpx_decrypt_cb,
    pub decrypt_state: *mut ::core::ffi::c_void,
    pub restart_threads: i32,
}
pub type vpx_decrypt_cb = Option<unsafe fn(
        *mut ::core::ffi::c_void,
        *const ::core::ffi::c_uchar,
        *mut ::core::ffi::c_uchar,
        i32,
    ) -> (),
>;
pub type semaphore_t = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DECODETHREAD_DATA {
    pub ithread: i32,
    pub ptr1: *mut ::core::ffi::c_void,
    pub ptr2: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MB_ROW_DEC {
    pub mbd: MACROBLOCKD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpx_atomic_int {
    pub value: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FRAGMENT_DATA {
    pub enabled: i32,
    pub count: ::core::ffi::c_uint,
    pub ptrs: [*const ::core::ffi::c_uchar; 9],
    pub sizes: [::core::ffi::c_uint; 9],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8D_CONFIG {
    pub Width: i32,
    pub Height: i32,
    pub Version: i32,
    pub postprocess: i32,
    pub max_threads: i32,
    pub error_concealment: i32,
}
pub type BOOL_DECODER = vp8_reader;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vp8_reader {
    pub user_buffer_end: *const ::core::ffi::c_uchar,
    pub user_buffer: *const ::core::ffi::c_uchar,
    pub value: VP8_BD_VALUE,
    pub count: i32,
    pub range: ::core::ffi::c_uint,
    pub decrypt_cb: vpx_decrypt_cb,
    pub decrypt_state: *mut ::core::ffi::c_void,
}
pub type VP8_BD_VALUE = size_t;
pub type VP8_COMMON = VP8Common;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Common {
    pub error: vpx_internal_error_info,
    pub Y1dequant: [[::core::ffi::c_short; 2]; 128],
    pub Y2dequant: [[::core::ffi::c_short; 2]; 128],
    pub UVdequant: [[::core::ffi::c_short; 2]; 128],
    pub Width: i32,
    pub Height: i32,
    pub horiz_scale: i32,
    pub vert_scale: i32,
    pub clamp_type: CLAMP_TYPE,
    pub frame_to_show: *mut YV12_BUFFER_CONFIG,
    pub yv12_fb: [YV12_BUFFER_CONFIG; 4],
    pub fb_idx_ref_cnt: [i32; 4],
    pub new_fb_idx: i32,
    pub lst_fb_idx: i32,
    pub gld_fb_idx: i32,
    pub alt_fb_idx: i32,
    pub temp_scale_frame: YV12_BUFFER_CONFIG,
    pub last_frame_type: FRAME_TYPE,
    pub frame_type: FRAME_TYPE,
    pub show_frame: i32,
    pub frame_flags: i32,
    pub MBs: i32,
    pub mb_rows: i32,
    pub mb_cols: i32,
    pub mode_info_stride: i32,
    pub mb_no_coeff_skip: i32,
    pub no_lpf: i32,
    pub use_bilinear_mc_filter: i32,
    pub full_pixel: i32,
    pub base_qindex: i32,
    pub y1dc_delta_q: i32,
    pub y2dc_delta_q: i32,
    pub y2ac_delta_q: i32,
    pub uvdc_delta_q: i32,
    pub uvac_delta_q: i32,
    pub mip: *mut MODE_INFO,
    pub mi: *mut MODE_INFO,
    pub show_frame_mi: *mut MODE_INFO,
    pub filter_type: LOOPFILTERTYPE,
    pub lf_info: loop_filter_info_n,
    pub filter_level: i32,
    pub last_sharpness_level: i32,
    pub sharpness_level: i32,
    pub refresh_last_frame: i32,
    pub refresh_golden_frame: i32,
    pub refresh_alt_ref_frame: i32,
    pub copy_buffer_to_gf: i32,
    pub copy_buffer_to_arf: i32,
    pub refresh_entropy_probs: i32,
    pub ref_frame_sign_bias: [i32; 4],
    pub above_context: *mut ENTROPY_CONTEXT_PLANES,
    pub left_context: ENTROPY_CONTEXT_PLANES,
    pub lfc: FRAME_CONTEXT,
    pub fc: FRAME_CONTEXT,
    pub current_video_frame: ::core::ffi::c_uint,
    pub version: i32,
    pub multi_token_partition: TOKEN_PARTITION,
    pub processor_core_count: i32,
}
pub type TOKEN_PARTITION = ::core::ffi::c_uint;
pub const EIGHT_PARTITION: TOKEN_PARTITION = 3;
pub const FOUR_PARTITION: TOKEN_PARTITION = 2;
pub const TWO_PARTITION: TOKEN_PARTITION = 1;
pub const ONE_PARTITION: TOKEN_PARTITION = 0;
pub type FRAME_CONTEXT = frame_contexts;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frame_contexts {
    pub bmode_prob: [vp8_prob; 9],
    pub ymode_prob: [vp8_prob; 4],
    pub uv_mode_prob: [vp8_prob; 3],
    pub sub_mv_ref_prob: [vp8_prob; 3],
    pub coef_probs: [[[[vp8_prob; 11]; 3]; 8]; 4],
    pub mvc: [MV_CONTEXT; 2],
}
pub type MV_CONTEXT = mv_context;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mv_context {
    pub prob: [vp8_prob; 19],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_info_n {
    pub mblim: [[::core::ffi::c_uchar; 16]; 64],
    pub blim: [[::core::ffi::c_uchar; 16]; 64],
    pub lim: [[::core::ffi::c_uchar; 16]; 64],
    pub hev_thr: [[::core::ffi::c_uchar; 16]; 4],
    pub lvl: [[[::core::ffi::c_uchar; 4]; 4]; 4],
    pub hev_thr_lut: [[::core::ffi::c_uchar; 64]; 2],
    pub mode_lf_lut: [::core::ffi::c_uchar; 10],
}
pub type LOOPFILTERTYPE = ::core::ffi::c_uint;
pub const SIMPLE_LOOPFILTER: LOOPFILTERTYPE = 1;
pub const NORMAL_LOOPFILTER: LOOPFILTERTYPE = 0;
pub type CLAMP_TYPE = ::core::ffi::c_uint;
pub const RECON_CLAMP_NOTREQUIRED: CLAMP_TYPE = 1;
pub const RECON_CLAMP_REQUIRED: CLAMP_TYPE = 0;
pub type ProbaArray = *const [[uint8_t; 11]; 3];
pub const CHAR_BIT: i32 = 8 as i32;
pub const VP8_BD_VALUE_SIZE: i32 =
    ::core::mem::size_of::<VP8_BD_VALUE>() as i32 * CHAR_BIT;
unsafe fn vp8dx_decode_bool(
    mut br: *mut BOOL_DECODER,
    mut probability: i32,
) -> i32 {
    unsafe {
        let mut bit: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut value: VP8_BD_VALUE = 0;
        let mut split: ::core::ffi::c_uint = 0;
        let mut bigsplit: VP8_BD_VALUE = 0;
        let mut count: i32 = 0;
        let mut range: ::core::ffi::c_uint = 0;
        split = (1 as ::core::ffi::c_uint).wrapping_add(
            (*br)
                .range
                .wrapping_sub(1 as ::core::ffi::c_uint)
                .wrapping_mul(probability as ::core::ffi::c_uint)
                >> 8 as i32,
        );
        if (*br).count < 0 as i32 {
            vp8dx_bool_decoder_fill(br);
        }
        value = (*br).value;
        count = (*br).count;
        bigsplit = (split as VP8_BD_VALUE) << (VP8_BD_VALUE_SIZE - 8 as i32);
        range = split;
        if value >= bigsplit {
            range = (*br).range.wrapping_sub(split);
            value = value.wrapping_sub(bigsplit);
            bit = 1 as ::core::ffi::c_uint;
        }
        let shift: ::core::ffi::c_uchar = vp8_norm[range as ::core::ffi::c_uchar as usize];
        range <<= shift as i32;
        value <<= shift as i32;
        count -= shift as i32;
        (*br).value = value;
        (*br).count = count;
        (*br).range = range;
        bit as i32
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_reset_mb_tokens_context(mut x: *mut MACROBLOCKD) {
    unsafe {
        let mut a_ctx: *mut ENTROPY_CONTEXT = (*x).above_context as *mut ENTROPY_CONTEXT;
        let mut l_ctx: *mut ENTROPY_CONTEXT = (*x).left_context as *mut ENTROPY_CONTEXT;
        memset(
            a_ctx as *mut ::core::ffi::c_void,
            0 as i32,
            (::core::mem::size_of::<ENTROPY_CONTEXT_PLANES>() as size_t).wrapping_sub(1 as size_t),
        );
        memset(
            l_ctx as *mut ::core::ffi::c_void,
            0 as i32,
            (::core::mem::size_of::<ENTROPY_CONTEXT_PLANES>() as size_t).wrapping_sub(1 as size_t),
        );
        if (*(*x).mode_info_context).mbmi.is_4x4 == 0 {
            let fresh0 = &mut *l_ctx.offset(8 as i32 as isize);
            *fresh0 = 0 as ENTROPY_CONTEXT;
            *a_ctx.offset(8 as i32 as isize) = *fresh0;
        }
    }
}
static mut kBands: [uint8_t; 17] = [
    0 as i32 as uint8_t,
    1 as i32 as uint8_t,
    2 as i32 as uint8_t,
    3 as i32 as uint8_t,
    6 as i32 as uint8_t,
    4 as i32 as uint8_t,
    5 as i32 as uint8_t,
    6 as i32 as uint8_t,
    6 as i32 as uint8_t,
    6 as i32 as uint8_t,
    6 as i32 as uint8_t,
    6 as i32 as uint8_t,
    6 as i32 as uint8_t,
    6 as i32 as uint8_t,
    6 as i32 as uint8_t,
    7 as i32 as uint8_t,
    0 as i32 as uint8_t,
];
static mut kCat3: [uint8_t; 4] = [
    173 as i32 as uint8_t,
    148 as i32 as uint8_t,
    140 as i32 as uint8_t,
    0 as i32 as uint8_t,
];
static mut kCat4: [uint8_t; 5] = [
    176 as i32 as uint8_t,
    155 as i32 as uint8_t,
    140 as i32 as uint8_t,
    135 as i32 as uint8_t,
    0 as i32 as uint8_t,
];
static mut kCat5: [uint8_t; 6] = [
    180 as i32 as uint8_t,
    157 as i32 as uint8_t,
    141 as i32 as uint8_t,
    134 as i32 as uint8_t,
    130 as i32 as uint8_t,
    0 as i32 as uint8_t,
];
static mut kCat6: [uint8_t; 12] = [
    254 as i32 as uint8_t,
    254 as i32 as uint8_t,
    243 as i32 as uint8_t,
    230 as i32 as uint8_t,
    196 as i32 as uint8_t,
    177 as i32 as uint8_t,
    153 as i32 as uint8_t,
    140 as i32 as uint8_t,
    133 as i32 as uint8_t,
    130 as i32 as uint8_t,
    129 as i32 as uint8_t,
    0 as i32 as uint8_t,
];
static mut kCat3456: [*const uint8_t; 4] = {
    [
        &raw const kCat3 as *const uint8_t,
        &raw const kCat4 as *const uint8_t,
        &raw const kCat5 as *const uint8_t,
        &raw const kCat6 as *const uint8_t,
    ]
};
static mut kZigzag: [uint8_t; 16] = [
    0 as i32 as uint8_t,
    1 as i32 as uint8_t,
    4 as i32 as uint8_t,
    8 as i32 as uint8_t,
    5 as i32 as uint8_t,
    2 as i32 as uint8_t,
    3 as i32 as uint8_t,
    6 as i32 as uint8_t,
    9 as i32 as uint8_t,
    12 as i32 as uint8_t,
    13 as i32 as uint8_t,
    10 as i32 as uint8_t,
    7 as i32 as uint8_t,
    11 as i32 as uint8_t,
    14 as i32 as uint8_t,
    15 as i32 as uint8_t,
];
unsafe fn GetSigned(
    mut br: *mut BOOL_DECODER,
    mut value_to_sign: i32,
) -> i32 {
    unsafe {
        let mut split: i32 = ((*br).range.wrapping_add(1 as ::core::ffi::c_uint)
            >> 1 as i32)
            as i32;
        let mut bigsplit: VP8_BD_VALUE =
            (split as VP8_BD_VALUE) << (VP8_BD_VALUE_SIZE - 8 as i32);
        let mut v: i32 = 0;
        if (*br).count < 0 as i32 {
            vp8dx_bool_decoder_fill(br);
        }
        if (*br).value < bigsplit {
            (*br).range = split as ::core::ffi::c_uint;
            v = value_to_sign;
        } else {
            (*br).range = (*br).range.wrapping_sub(split as ::core::ffi::c_uint);
            (*br).value = (*br).value.wrapping_sub(bigsplit);
            v = -value_to_sign;
        }
        (*br).range = (*br).range.wrapping_add((*br).range);
        (*br).value = (*br).value.wrapping_add((*br).value);
        (*br).count -= 1;
        v
    }
}
unsafe fn GetCoeffs(
    mut br: *mut BOOL_DECODER,
    mut prob: ProbaArray,
    mut ctx: i32,
    mut n: i32,
    mut out: *mut int16_t,
) -> i32 {
    unsafe {
        let mut p: *const uint8_t = &raw const *(&raw const *prob.offset(n as isize)
            as *const [uint8_t; 11])
            .offset(ctx as isize) as *const uint8_t;
        if vp8dx_decode_bool(
            br,
            *p.offset(0 as i32 as isize) as i32,
        ) == 0
        {
            return 0 as i32;
        }
        loop {
            n += 1;
            if vp8dx_decode_bool(
                br,
                *p.offset(1 as i32 as isize) as i32,
            ) == 0
            {
                p = &raw const *(&raw const *prob
                    .offset(*(&raw const kBands as *const uint8_t).offset(n as isize) as isize)
                    as *const [uint8_t; 11])
                    .offset(0 as i32 as isize) as *const uint8_t;
            } else {
                let mut v: i32 = 0;
                let mut j: i32 = 0;
                if vp8dx_decode_bool(
                    br,
                    *p.offset(2 as i32 as isize) as i32,
                ) == 0
                {
                    p = &raw const *(&raw const *prob
                        .offset(*(&raw const kBands as *const uint8_t).offset(n as isize) as isize)
                        as *const [uint8_t; 11])
                        .offset(1 as i32 as isize)
                        as *const uint8_t;
                    v = 1 as i32;
                } else {
                    if vp8dx_decode_bool(
                        br,
                        *p.offset(3 as i32 as isize) as i32,
                    ) == 0
                    {
                        if vp8dx_decode_bool(
                            br,
                            *p.offset(4 as i32 as isize) as i32,
                        ) == 0
                        {
                            v = 2 as i32;
                        } else {
                            v = 3 as i32
                                + vp8dx_decode_bool(
                                    br,
                                    *p.offset(5 as i32 as isize)
                                        as i32,
                                );
                        }
                    } else if vp8dx_decode_bool(
                        br,
                        *p.offset(6 as i32 as isize) as i32,
                    ) == 0
                    {
                        if vp8dx_decode_bool(
                            br,
                            *p.offset(7 as i32 as isize) as i32,
                        ) == 0
                        {
                            v = 5 as i32
                                + vp8dx_decode_bool(br, 159 as i32);
                        } else {
                            v = 7 as i32
                                + 2 as i32
                                    * vp8dx_decode_bool(br, 165 as i32);
                            v += vp8dx_decode_bool(br, 145 as i32);
                        }
                    } else {
                        let mut tab: *const uint8_t = ::core::ptr::null::<uint8_t>();
                        let bit1: i32 = vp8dx_decode_bool(
                            br,
                            *p.offset(8 as i32 as isize) as i32,
                        )
                            as i32;
                        let bit0: i32 = vp8dx_decode_bool(
                            br,
                            *p.offset((9 as i32 + bit1) as isize)
                                as i32,
                        )
                            as i32;
                        let cat: i32 = 2 as i32 * bit1 + bit0;
                        v = 0 as i32;
                        tab = kCat3456[cat as usize];
                        while *tab != 0 {
                            v += v + vp8dx_decode_bool(br, *tab as i32);
                            tab = tab.offset(1);
                        }
                        v += 3 as i32 + ((8 as i32) << cat);
                    }
                    p = &raw const *(&raw const *prob
                        .offset(*(&raw const kBands as *const uint8_t).offset(n as isize) as isize)
                        as *const [uint8_t; 11])
                        .offset(2 as i32 as isize)
                        as *const uint8_t;
                }
                j = kZigzag[(n - 1 as i32) as usize] as i32;
                *out.offset(j as isize) = GetSigned(br, v) as int16_t;
                if n == 16 as i32
                    || vp8dx_decode_bool(
                        br,
                        *p.offset(0 as i32 as isize) as i32,
                    ) == 0
                {
                    return n;
                }
            }
            if n == 16 as i32 {
                return 16 as i32;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_decode_mb_tokens(
    mut dx: *mut VP8D_COMP,
    mut x: *mut MACROBLOCKD,
) -> i32 {
    unsafe {
        let mut bc: *mut BOOL_DECODER = (*x).current_bc as *mut BOOL_DECODER;
        let fc: *const FRAME_CONTEXT = &raw mut (*dx).common.fc;
        let mut eobs: *mut ::core::ffi::c_char = &raw mut (*x).eobs as *mut ::core::ffi::c_char;
        let mut i: i32 = 0;
        let mut nonzeros: i32 = 0;
        let mut eobtotal: i32 = 0 as i32;
        let mut qcoeff_ptr: *mut ::core::ffi::c_short =
            ::core::ptr::null_mut::<::core::ffi::c_short>();
        let mut coef_probs: ProbaArray = ::core::ptr::null::<[[uint8_t; 11]; 3]>();
        let mut a_ctx: *mut ENTROPY_CONTEXT = (*x).above_context as *mut ENTROPY_CONTEXT;
        let mut l_ctx: *mut ENTROPY_CONTEXT = (*x).left_context as *mut ENTROPY_CONTEXT;
        let mut a: *mut ENTROPY_CONTEXT = ::core::ptr::null_mut::<ENTROPY_CONTEXT>();
        let mut l: *mut ENTROPY_CONTEXT = ::core::ptr::null_mut::<ENTROPY_CONTEXT>();
        let mut skip_dc: i32 = 0 as i32;
        qcoeff_ptr = (&raw mut (*x).qcoeff as *mut ::core::ffi::c_short)
            .offset(0 as i32 as isize)
            as *mut ::core::ffi::c_short;
        if (*(*x).mode_info_context).mbmi.is_4x4 == 0 {
            a = a_ctx.offset(8 as i32 as isize);
            l = l_ctx.offset(8 as i32 as isize);
            coef_probs = &raw const *(&raw const (*fc).coef_probs
                as *const [[[vp8_prob; 11]; 3]; 8])
                .offset(1 as i32 as isize)
                as *const [[vp8_prob; 11]; 3] as ProbaArray;
            nonzeros = GetCoeffs(
                bc,
                coef_probs,
                *a as i32 + *l as i32,
                0 as i32,
                qcoeff_ptr.offset((24 as i32 * 16 as i32) as isize),
            );
            *l = (nonzeros > 0 as i32) as i32 as ENTROPY_CONTEXT;
            *a = *l;
            *eobs.offset(24 as i32 as isize) = nonzeros as ::core::ffi::c_char;
            eobtotal += nonzeros - 16 as i32;
            coef_probs = &raw const *(&raw const (*fc).coef_probs
                as *const [[[vp8_prob; 11]; 3]; 8])
                .offset(0 as i32 as isize)
                as *const [[vp8_prob; 11]; 3] as ProbaArray;
            skip_dc = 1 as i32;
        } else {
            coef_probs = &raw const *(&raw const (*fc).coef_probs
                as *const [[[vp8_prob; 11]; 3]; 8])
                .offset(3 as i32 as isize)
                as *const [[vp8_prob; 11]; 3] as ProbaArray;
            skip_dc = 0 as i32;
        }
        i = 0 as i32;
        while i < 16 as i32 {
            a = a_ctx.offset((i & 3 as i32) as isize);
            l = l_ctx.offset(((i & 0xc as i32) >> 2 as i32) as isize);
            nonzeros = GetCoeffs(
                bc,
                coef_probs,
                *a as i32 + *l as i32,
                skip_dc,
                qcoeff_ptr as *mut int16_t,
            );
            *l = (nonzeros > 0 as i32) as i32 as ENTROPY_CONTEXT;
            *a = *l;
            nonzeros += skip_dc;
            *eobs.offset(i as isize) = nonzeros as ::core::ffi::c_char;
            eobtotal += nonzeros;
            qcoeff_ptr = qcoeff_ptr.offset(16 as i32 as isize);
            i += 1;
        }
        coef_probs = &raw const *(&raw const (*fc).coef_probs as *const [[[vp8_prob; 11]; 3]; 8])
            .offset(2 as i32 as isize)
            as *const [[vp8_prob; 11]; 3] as ProbaArray;
        a_ctx = a_ctx.offset(4 as i32 as isize);
        l_ctx = l_ctx.offset(4 as i32 as isize);
        i = 16 as i32;
        while i < 24 as i32 {
            a = a_ctx
                .offset(
                    (((i > 19 as i32) as i32)
                        << 1 as i32) as isize,
                )
                .offset((i & 1 as i32) as isize);
            l = l_ctx
                .offset(
                    (((i > 19 as i32) as i32)
                        << 1 as i32) as isize,
                )
                .offset(
                    (i & 3 as i32 > 1 as i32) as i32
                        as isize,
                );
            nonzeros = GetCoeffs(
                bc,
                coef_probs,
                *a as i32 + *l as i32,
                0 as i32,
                qcoeff_ptr as *mut int16_t,
            );
            *l = (nonzeros > 0 as i32) as i32 as ENTROPY_CONTEXT;
            *a = *l;
            *eobs.offset(i as isize) = nonzeros as ::core::ffi::c_char;
            eobtotal += nonzeros;
            qcoeff_ptr = qcoeff_ptr.offset(16 as i32 as isize);
            i += 1;
        }
        eobtotal
    }
}
