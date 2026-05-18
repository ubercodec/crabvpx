unsafe extern "C" {
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn vpx_d117_predictor_4x4_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_d135_predictor_4x4_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_d153_predictor_4x4_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_d207_predictor_4x4_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_d45e_predictor_4x4_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_d63e_predictor_4x4_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_dc_predictor_4x4_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_he_predictor_4x4_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_tm_predictor_4x4_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn vpx_ve_predictor_4x4_c(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
}
pub type __darwin_ptrdiff_t = isize;
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type ptrdiff_t = __darwin_ptrdiff_t;
pub type uint8_t = u8;
pub use crate::vp8::common::types::*;
pub type intra_pred_fn =
    Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> ()>;
static pred: [intra_pred_fn; 10] = [
    Some(vpx_dc_predictor_4x4_neon),
    Some(vpx_tm_predictor_4x4_neon),
    Some(vpx_ve_predictor_4x4_c),
    Some(vpx_he_predictor_4x4_c),
    Some(vpx_d45e_predictor_4x4_c),
    Some(vpx_d135_predictor_4x4_neon),
    Some(vpx_d117_predictor_4x4_neon),
    Some(vpx_d63e_predictor_4x4_c),
    Some(vpx_d153_predictor_4x4_neon),
    Some(vpx_d207_predictor_4x4_neon),
];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_intra4x4_predict(
    mut above: *mut ::core::ffi::c_uchar,
    mut yleft: *mut ::core::ffi::c_uchar,
    mut left_stride: ::core::ffi::c_int,
    mut b_mode: B_PREDICTION_MODE,
    mut dst: *mut ::core::ffi::c_uchar,
    mut dst_stride: ::core::ffi::c_int,
    mut top_left: ::core::ffi::c_uchar,
) { unsafe {
    let mut Aboveb: [::core::ffi::c_uchar; 12] = [0; 12];
    let mut Above: *mut ::core::ffi::c_uchar =
        (&raw mut Aboveb as *mut ::core::ffi::c_uchar).offset(4 as ::core::ffi::c_int as isize);
    let mut Left: [::core::ffi::c_uchar; 8] = [0; 8];
    Left[0 as ::core::ffi::c_int as usize] = *yleft.offset(0 as ::core::ffi::c_int as isize);
    Left[1 as ::core::ffi::c_int as usize] = *yleft.offset(left_stride as isize);
    Left[2 as ::core::ffi::c_int as usize] =
        *yleft.offset((2 as ::core::ffi::c_int * left_stride) as isize);
    Left[3 as ::core::ffi::c_int as usize] =
        *yleft.offset((3 as ::core::ffi::c_int * left_stride) as isize);
    memcpy(
        Above as *mut ::core::ffi::c_void,
        above as *const ::core::ffi::c_void,
        8 as size_t,
    );
    *Above.offset(-(1 as ::core::ffi::c_int) as isize) = top_left;
    pred[b_mode as usize].expect("non-null function pointer")(
        dst as *mut uint8_t,
        dst_stride as ptrdiff_t,
        Above,
        &raw mut Left as *mut ::core::ffi::c_uchar,
    );
}}
