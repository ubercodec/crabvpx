unsafe extern "C" {
    fn vpx_dc_128_predictor_16x16_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_dc_128_predictor_8x8_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_dc_left_predictor_16x16_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_dc_left_predictor_8x8_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_dc_predictor_16x16_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_dc_predictor_8x8_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_dc_top_predictor_16x16_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_dc_top_predictor_8x8_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_h_predictor_16x16_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_h_predictor_8x8_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_tm_predictor_16x16_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_tm_predictor_8x8_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_v_predictor_16x16_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_v_predictor_8x8_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
}
pub type __darwin_ptrdiff_t = isize;
pub type __darwin_size_t = usize;
pub type ptrdiff_t = __darwin_ptrdiff_t;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub use crate::vp8::common::types::*;

pub type vpx_color_range_t = vpx_color_range;
pub type vpx_color_range = ::core::ffi::c_uint;
pub const VPX_CR_FULL_RANGE: vpx_color_range = 1;
pub const VPX_CR_STUDIO_RANGE: vpx_color_range = 0;
pub type vpx_color_space_t = vpx_color_space;
pub type vpx_color_space = ::core::ffi::c_uint;
pub const VPX_CS_SRGB: vpx_color_space = 7;
pub const VPX_CS_RESERVED: vpx_color_space = 6;
pub const VPX_CS_BT_2020: vpx_color_space = 5;
pub const VPX_CS_SMPTE_240: vpx_color_space = 4;
pub const VPX_CS_SMPTE_170: vpx_color_space = 3;
pub const VPX_CS_BT_709: vpx_color_space = 2;
pub const VPX_CS_BT_601: vpx_color_space = 1;
pub const VPX_CS_UNKNOWN: vpx_color_space = 0;

pub type intra_pred_fn =
    Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
pub const SIZE_16: C2RustUnnamed = 0;
pub const SIZE_8: C2RustUnnamed = 1;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const NUM_SIZES: C2RustUnnamed = 2;

static pred: [[intra_pred_fn; 2]; 4] = [
    [None, None],
    [
        Some(vpx_v_predictor_16x16_neon),
        Some(vpx_v_predictor_8x8_neon),
    ],
    [
        Some(vpx_h_predictor_16x16_neon),
        Some(vpx_h_predictor_8x8_neon),
    ],
    [
        Some(vpx_tm_predictor_16x16_neon),
        Some(vpx_tm_predictor_8x8_neon),
    ],
];

static dc_pred: [[[intra_pred_fn; 2]; 2]; 2] = [
    [
        [
            Some(vpx_dc_128_predictor_16x16_neon),
            Some(vpx_dc_128_predictor_8x8_neon),
        ],
        [
            Some(vpx_dc_top_predictor_16x16_neon),
            Some(vpx_dc_top_predictor_8x8_neon),
        ],
    ],
    [
        [
            Some(vpx_dc_left_predictor_16x16_neon),
            Some(vpx_dc_left_predictor_8x8_neon),
        ],
        [
            Some(vpx_dc_predictor_16x16_neon),
            Some(vpx_dc_predictor_8x8_neon),
        ],
    ],
];
pub fn vp8_build_intra_predictors_mby_safe(
    x: &MACROBLOCKD,
    yabove: &[u8; 17],
    yleft: &[u8; 16],
    ypred_slice: &mut [u8],
    y_stride: usize,
) {
    let mode = x.mode_info().mbmi.mode as MB_PREDICTION_MODE;
    let fn_0 = if mode as ::core::ffi::c_uint == DC_PRED as ::core::ffi::c_uint {
        dc_pred[x.left_available as usize][x.up_available as usize]
            [SIZE_16 as usize]
    } else {
        pred[mode as usize][SIZE_16 as usize]
    };
    if let Some(pred_fn) = fn_0 {
        unsafe {
            pred_fn(
                ypred_slice.as_mut_ptr(),
                y_stride as ptrdiff_t,
                yabove[1..].as_ptr(),
                yleft.as_ptr(),
            );
        }
    }
}
pub fn vp8_build_intra_predictors_mbuv_safe(
    x: &MACROBLOCKD,
    uabove: &[u8; 9],
    vabove: &[u8; 9],
    uleft: &[u8; 8],
    vleft: &[u8; 8],
    upred_slice: &mut [u8],
    vpred_slice: &mut [u8],
    uv_stride: usize,
) {
    let uvmode = x.mode_info().mbmi.uv_mode as MB_PREDICTION_MODE;
    let fn_0 = if uvmode as ::core::ffi::c_uint == DC_PRED as ::core::ffi::c_uint {
        dc_pred[x.left_available as usize][x.up_available as usize]
            [SIZE_8 as usize]
    } else {
        pred[uvmode as usize][SIZE_8 as usize]
    };
    if let Some(pred_fn) = fn_0 {
        unsafe {
            pred_fn(
                upred_slice.as_mut_ptr(),
                uv_stride as ptrdiff_t,
                uabove[1..].as_ptr(),
                uleft.as_ptr(),
            );
            pred_fn(
                vpred_slice.as_mut_ptr(),
                uv_stride as ptrdiff_t,
                vabove[1..].as_ptr(),
                vleft.as_ptr(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vp8_init_intra_predictors() {}

pub fn intra_prediction_down_copy(
    xd: &mut MACROBLOCKD,
) {
    let dst_stride = xd.dst.y_stride as usize;
    let border = xd.dst.border as usize;

    let y_slice = xd.dst.y_slice_mut_safe();

    let base_idx = (border - 1) * dst_stride + border + 16;

    let mut src_bytes = [0u8; 4];
    src_bytes.copy_from_slice(&y_slice[base_idx..base_idx + 4]);

    let idx0 = base_idx + 4 * dst_stride;
    let idx1 = base_idx + 8 * dst_stride;
    let idx2 = base_idx + 12 * dst_stride;

    y_slice[idx0..idx0 + 4].copy_from_slice(&src_bytes);
    y_slice[idx1..idx1 + 4].copy_from_slice(&src_bytes);
    y_slice[idx2..idx2 + 4].copy_from_slice(&src_bytes);
}

