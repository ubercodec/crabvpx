//! Inverse transforms — port of `vp8/common/idctllm.c`.
//!
//! 4×4 inverse DCT (full + DC-only) and 4×4 inverse Walsh-Hadamard (full +
//! DC-only, for the Y2 second-order block); each adds onto the predictor in
//! place. NEON variants in [`crate::vp8::common::simd`] stay bit-exact.

const COSPI8SQRT2MINUS1: i32 = 20091;
const SINPI8SQRT2: i32 = 35468;

/// Inverse 4x4 DCT added in place: the predictor is the current `dst` content,
/// so it is read straight from `dst` instead of a separate copy. Each output
/// pixel depends only on its own predictor pixel, so the read-then-write is
/// bit-identical to the previous copy-the-predictor form.
pub fn vp8_short_idct4x4llm_safe(input: &[i16; 16], dst: &mut [u8], dst_stride: i32) {
    let mut output = [0i16; 16];

    // First pass: process columns
    for i in 0..4 {
        let a1 = input[i] as i32 + input[i + 8] as i32;
        let b1 = input[i] as i32 - input[i + 8] as i32;

        let temp1 = (input[i + 4] as i32 * SINPI8SQRT2) >> 16;
        let temp2 = input[i + 12] as i32 + ((input[i + 12] as i32 * COSPI8SQRT2MINUS1) >> 16);
        let c1 = temp1 - temp2;

        let temp1 = input[i + 4] as i32 + ((input[i + 4] as i32 * COSPI8SQRT2MINUS1) >> 16);
        let temp2 = (input[i + 12] as i32 * SINPI8SQRT2) >> 16;
        let d1 = temp1 + temp2;

        output[i] = (a1 + d1) as i16;
        output[i + 12] = (a1 - d1) as i16;
        output[i + 4] = (b1 + c1) as i16;
        output[i + 8] = (b1 - c1) as i16;
    }

    // Second pass: process rows
    let temp_output = output;
    for i in 0..4 {
        let a1 = temp_output[i * 4] as i32 + temp_output[i * 4 + 2] as i32;
        let b1 = temp_output[i * 4] as i32 - temp_output[i * 4 + 2] as i32;

        let temp1 = (temp_output[i * 4 + 1] as i32 * SINPI8SQRT2) >> 16;
        let temp2 = temp_output[i * 4 + 3] as i32
            + ((temp_output[i * 4 + 3] as i32 * COSPI8SQRT2MINUS1) >> 16);
        let c1 = temp1 - temp2;

        let temp1 = temp_output[i * 4 + 1] as i32
            + ((temp_output[i * 4 + 1] as i32 * COSPI8SQRT2MINUS1) >> 16);
        let temp2 = (temp_output[i * 4 + 3] as i32 * SINPI8SQRT2) >> 16;
        let d1 = temp1 + temp2;

        output[i * 4] = ((a1 + d1 + 4) >> 3) as i16;
        output[i * 4 + 3] = ((a1 - d1 + 4) >> 3) as i16;
        output[i * 4 + 1] = ((b1 + c1 + 4) >> 3) as i16;
        output[i * 4 + 2] = ((b1 - c1 + 4) >> 3) as i16;
    }

    // Add predictor (read in place from dst) and clamp.
    for r in 0..4 {
        for c in 0..4 {
            let idx = r * dst_stride as usize + c;
            let a = output[r * 4 + c] as i32 + dst[idx] as i32;
            dst[idx] = a.clamp(0, 255) as u8;
        }
    }
}

/// Add the DC-only inverse transform (a single rounded DC term) to the
/// predictor and clamp, in place. The predictor is the current `dst` content,
/// so it is read straight from `dst` (each pixel depends only on itself, so the
/// read-then-write is bit-identical to the previous copy-the-predictor form).
/// Dispatches to the NEON kernel on aarch64; scalar elsewhere.
pub fn vp8_dc_only_idct_add_safe(input_dc: i16, dst: &mut [u8], dst_stride: i32) {
    #[cfg(target_arch = "aarch64")]
    {
        crate::vp8::common::simd::neon::vp8_dc_only_idct_add_neon(input_dc, dst, dst_stride);
    }
    #[cfg(not(target_arch = "aarch64"))]
    vp8_dc_only_idct_add_scalar(input_dc, dst, dst_stride);
}

pub fn vp8_dc_only_idct_add_scalar(input_dc: i16, dst: &mut [u8], dst_stride: i32) {
    let a1 = (input_dc as i32 + 4) >> 3;
    for r in 0..4 {
        for c in 0..4 {
            let idx = r * dst_stride as usize + c;
            let a = a1 + dst[idx] as i32;
            dst[idx] = a.clamp(0, 255) as u8;
        }
    }
}

/// `vp8_short_inv_walsh4x4_1_c` — vp8/common/idctllm.c:177. DC-only inverse WHT
/// for the Y2 block: scatters one rounded DC term across all 16 outputs.
pub fn vp8_short_inv_walsh4x4_1_safe(input: &[i16], mb_dqcoeff: &mut [i16]) {
    let a1 = (input[0] as i32 + 3) >> 3;
    for i in 0..16 {
        mb_dqcoeff[i * 16] = a1 as i16;
    }
}

/// `vp8_short_inv_walsh4x4_c` — vp8/common/idctllm.c:127. Full inverse WHT for
/// the Y2 second-order block; writes the 16 second-order DC coefficients.
pub fn vp8_short_inv_walsh4x4_safe(input: &[i16; 16], mb_dqcoeff: &mut [i16]) {
    let mut output = [0i16; 16];

    // First pass: process columns
    for i in 0..4 {
        let a1 = input[i] as i32 + input[i + 12] as i32;
        let b1 = input[i + 4] as i32 + input[i + 8] as i32;
        let c1 = input[i + 4] as i32 - input[i + 8] as i32;
        let d1 = input[i] as i32 - input[i + 12] as i32;

        output[i] = (a1 + b1) as i16;
        output[i + 4] = (c1 + d1) as i16;
        output[i + 8] = (a1 - b1) as i16;
        output[i + 12] = (d1 - c1) as i16;
    }

    // Second pass: process rows
    let temp_output = output;
    for i in 0..4 {
        let row = i * 4;
        let a1 = temp_output[row] as i32 + temp_output[row + 3] as i32;
        let b1 = temp_output[row + 1] as i32 + temp_output[row + 2] as i32;
        let c1 = temp_output[row + 1] as i32 - temp_output[row + 2] as i32;
        let d1 = temp_output[row] as i32 - temp_output[row + 3] as i32;

        let a2 = a1 + b1;
        let b2 = c1 + d1;
        let c2 = a1 - b1;
        let d2 = d1 - c1;

        output[row] = ((a2 + 3) >> 3) as i16;
        output[row + 1] = ((b2 + 3) >> 3) as i16;
        output[row + 2] = ((c2 + 3) >> 3) as i16;
        output[row + 3] = ((d2 + 3) >> 3) as i16;
    }

    // Write to mb_dqcoeff
    for i in 0..16 {
        mb_dqcoeff[i * 16] = output[i];
    }
}
