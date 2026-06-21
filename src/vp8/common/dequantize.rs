//! Inverse quantization — port of `vp8/common/dequantize.c`.
//!
//! Two kernels: [`vp8_dequantize_b_safe`] (standalone dequant, used for the Y2
//! second-order block) and [`vp8_dequant_idct_add_safe`] (fused dequant + 4×4
//! IDCT + add against the predictor, the common per-block path). The NEON
//! variants in [`crate::vp8::common::simd`] must stay bit-exact with these.

pub use crate::vp8::common::types::*;

/// `vp8_dequantize_b_c` — vp8/common/dequantize.c:16.
///
/// Multiplies the 16 quantized coefficients `q` by the dequant table `DQC`
/// into `dq`. In C the `short * short` product is promoted to `int` and
/// truncated back to `short` on store; the `as i32 … as i16` here reproduces
/// that wrap exactly.
pub fn vp8_dequantize_b_safe(q: &[i16], dq: &mut [i16], DQC: &[i16]) {
    for i in 0..16 {
        dq[i] = (q[i] as i32 * DQC[i] as i32) as i16;
    }
}

/// `vp8_dequant_idct_add_c` — vp8/common/dequantize.c:26.
///
/// Fused dequant + inverse transform + accumulate: dequantizes `input` in
/// place, runs the 4×4 IDCT adding the result onto `dest` (the existing
/// predictor pixels), then zeroes `input` for reuse. `stride` is the `dest`
/// row stride.
pub fn vp8_dequant_idct_add_safe(
    input: &mut [i16; 16],
    dq: &[i16; 16],
    dest: &mut [u8],
    stride: i32,
) {
    for i in 0..16 {
        input[i] = (dq[i] as i32 * input[i] as i32) as i16;
    }

    crate::vp8::common::idctllm::vp8_short_idct4x4llm_safe(input, dest, stride);

    input.fill(0);
}
