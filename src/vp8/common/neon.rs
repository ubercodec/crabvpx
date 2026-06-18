//! NEON SIMD kernels (aarch64), per ADR-014.
//!
//! This module is the only place the decoder carries SIMD `unsafe`. Each kernel
//! is a bit-exact twin of a scalar reference (here, `filter::filter_block2d_sixtap_safe`),
//! gated by `tests` below and by the end-to-end `differential_md5` suite. All
//! arithmetic is ordinary safe code: calling a NEON intrinsic from inside a
//! `#[target_feature(enable = "neon")]` fn is safe on Rust >= 1.87. The residual
//! `unsafe` is confined to register loads/stores (NEON is ARMv8-A baseline, so
//! the target-feature precondition always holds here).
#![cfg(target_arch = "aarch64")]

use core::arch::aarch64::*;

const HALF: i32 = 64; // VP8_FILTER_WEIGHT >> 1
const SHIFT: i32 = 7; // VP8_FILTER_SHIFT

/// Max intermediate size: (16 + 5) rows * 16 wide, plus slack for the 8-lane
/// vertical loads to read past a width-4 row without going out of bounds.
const MID_LEN: usize = 21 * 16 + 8;

/// Bit-exact NEON twin of [`crate::vp8::common::filter::filter_block2d_sixtap_safe`].
///
/// Two separable passes: horizontal 6-tap producing a clamped [0,255] i16
/// intermediate, then vertical 6-tap to the u8 destination. Falls back to the
/// scalar kernel if the source slice is too short for the (border-backed) NEON
/// over-reads, so it is always memory-safe.
pub(crate) fn filter_block2d_sixtap_neon(
    src: &[u8],
    src_stride: usize,
    dst: &mut [u8],
    dst_pitch: usize,
    width: usize,
    height: usize,
    h_filter: &[i16; 6],
    v_filter: &[i16; 6],
) {
    let ih = height + 5;
    // Horizontal pass reads up to `base + 13` per 8-col chunk; for the last
    // chunk that can exceed width+5 by a few bytes (covered by the frame border
    // in real use). Require it; otherwise use the scalar twin.
    let last_base = ((width + 7) / 8 - 1) * 8;
    let req_src = ih.saturating_sub(1) * src_stride + last_base + 13;
    let req_dst = height.saturating_sub(1) * dst_pitch + width;
    if src.len() < req_src || dst.len() < req_dst || width == 0 || height == 0 {
        crate::vp8::common::filter::filter_block2d_sixtap_safe(
            src, src_stride, dst, dst_pitch, width, height, h_filter, v_filter,
        );
        return;
    }

    let mut mid = [0i16; MID_LEN];
    // SAFETY: NEON is ARMv8-A baseline (discharges the `#[target_feature]`
    // precondition). The length checks above guarantee every load/store below
    // stays in bounds of `src`, `dst`, and the padded `mid` buffer.
    unsafe {
        sixtap_impl(
            src, src_stride, &mut mid, dst, dst_pitch, width, height, h_filter, v_filter,
        );
    }
}

#[target_feature(enable = "neon")]
fn sixtap_impl(
    src: &[u8],
    src_stride: usize,
    mid: &mut [i16],
    dst: &mut [u8],
    dst_pitch: usize,
    width: usize,
    height: usize,
    hf: &[i16; 6],
    vf: &[i16; 6],
) {
    let ih = height + 5;

    // ---- Horizontal pass: src(u8) -> mid(i16, clamped 0..255) ----
    for i in 0..ih {
        let row = i * src_stride;
        let mout = i * width;
        let mut c = 0;
        while c < width {
            let base = row + c;
            let mut acc_lo = vdupq_n_s32(HALF);
            let mut acc_hi = vdupq_n_s32(HALF);
            // 6 taps: src[base+k .. base+k+8]
            macro_rules! tap {
                ($k:literal) => {{
                    // SAFETY: base + k + 8 <= req_src by construction.
                    let v = unsafe { vld1_u8(src.as_ptr().add(base + $k)) };
                    let w = vreinterpretq_s16_u16(vmovl_u8(v));
                    let f = vdup_n_s16(hf[$k]);
                    acc_lo = vmlal_s16(acc_lo, vget_low_s16(w), f);
                    acc_hi = vmlal_s16(acc_hi, vget_high_s16(w), f);
                }};
            }
            tap!(0);
            tap!(1);
            tap!(2);
            tap!(3);
            tap!(4);
            tap!(5);
            let res = pack_clamped(acc_lo, acc_hi);
            store_i16(&mut mid[mout + c..], res, (width - c).min(8));
            c += 8;
        }
    }

    // ---- Vertical pass: mid(i16) -> dst(u8) ----
    for i in 0..height {
        let mut c = 0;
        while c < width {
            let mut acc_lo = vdupq_n_s32(HALF);
            let mut acc_hi = vdupq_n_s32(HALF);
            macro_rules! vtap {
                ($k:literal) => {{
                    // SAFETY: (i + k) < ih and the +8 slack on `mid` keeps the
                    // 8-lane load in bounds even when width == 4.
                    let m = unsafe { vld1q_s16(mid.as_ptr().add((i + $k) * width + c)) };
                    let f = vdup_n_s16(vf[$k]);
                    acc_lo = vmlal_s16(acc_lo, vget_low_s16(m), f);
                    acc_hi = vmlal_s16(acc_hi, vget_high_s16(m), f);
                }};
            }
            vtap!(0);
            vtap!(1);
            vtap!(2);
            vtap!(3);
            vtap!(4);
            vtap!(5);
            let res = pack_clamped(acc_lo, acc_hi); // i16x8 in [0,255]
            let bytes = vmovn_u16(vreinterpretq_u16_s16(res)); // u8x8
            store_u8(&mut dst[i * dst_pitch + c..], bytes, (width - c).min(8));
            c += 8;
        }
    }
}

/// `(acc + ... already has HALF) >> 7`, clamped to [0,255], as i16x8.
#[target_feature(enable = "neon")]
fn pack_clamped(acc_lo: int32x4_t, acc_hi: int32x4_t) -> int16x8_t {
    let lo = vshrq_n_s32::<SHIFT>(acc_lo);
    let hi = vshrq_n_s32::<SHIFT>(acc_hi);
    let zero = vdupq_n_s32(0);
    let max = vdupq_n_s32(255);
    let lo = vminq_s32(vmaxq_s32(lo, zero), max);
    let hi = vminq_s32(vmaxq_s32(hi, zero), max);
    vcombine_s16(vmovn_s32(lo), vmovn_s32(hi))
}

#[target_feature(enable = "neon")]
fn store_i16(dst: &mut [i16], v: int16x8_t, n: usize) {
    if n >= 8 {
        // SAFETY: caller guarantees dst has >= 8 elements when n >= 8.
        unsafe { vst1q_s16(dst.as_mut_ptr(), v) };
    } else {
        let mut tmp = [0i16; 8];
        unsafe { vst1q_s16(tmp.as_mut_ptr(), v) };
        dst[..n].copy_from_slice(&tmp[..n]);
    }
}

#[target_feature(enable = "neon")]
fn store_u8(dst: &mut [u8], v: uint8x8_t, n: usize) {
    if n >= 8 {
        // SAFETY: caller guarantees dst has >= 8 elements when n >= 8.
        unsafe { vst1_u8(dst.as_mut_ptr(), v) };
    } else {
        let mut tmp = [0u8; 8];
        unsafe { vst1_u8(tmp.as_mut_ptr(), v) };
        dst[..n].copy_from_slice(&tmp[..n]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vp8::common::filter::{filter_block2d_sixtap_safe, vp8_sub_pel_filters};

    // Tiny deterministic PRNG so the test needs no deps.
    fn lcg(state: &mut u64) -> u8 {
        *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (*state >> 33) as u8
    }

    fn check_size(width: usize, height: usize) {
        let stride = width + 16; // generous border
        let ih = height + 5;
        let src_len = (ih) * stride + 32;
        for xo in 0..8 {
            for yo in 0..8 {
                let hf = &vp8_sub_pel_filters[xo];
                let vf = &vp8_sub_pel_filters[yo];
                let mut seed = (xo * 131 + yo * 977 + width * 17 + height) as u64 + 1;
                let mut src = vec![0u8; src_len];
                for b in src.iter_mut() {
                    *b = lcg(&mut seed);
                }
                let mut dst_scalar = vec![0u8; height * width];
                let mut dst_neon = vec![0u8; height * width];
                filter_block2d_sixtap_safe(&src, stride, &mut dst_scalar, width, width, height, hf, vf);
                filter_block2d_sixtap_neon(&src, stride, &mut dst_neon, width, width, height, hf, vf);
                assert_eq!(
                    dst_scalar, dst_neon,
                    "mismatch {width}x{height} xo={xo} yo={yo}"
                );
            }
        }
    }

    #[test]
    fn neon_sixtap_matches_scalar_bit_exact() {
        check_size(4, 4);
        check_size(8, 4);
        check_size(8, 8);
        check_size(16, 16);
    }

    // Run with: cargo test --release --lib neon_sixtap_microbench -- --nocapture --ignored
    #[test]
    #[ignore]
    fn neon_sixtap_microbench() {
        use std::time::Instant;
        let (w, h) = (16usize, 16usize);
        let stride = w + 16;
        let src_len = (h + 5) * stride + 32;
        let mut seed = 12345u64;
        let mut src = vec![0u8; src_len];
        for b in src.iter_mut() {
            *b = lcg(&mut seed);
        }
        let hf = &vp8_sub_pel_filters[3];
        let vf = &vp8_sub_pel_filters[5];
        let mut dst = vec![0u8; h * w];
        let iters = 2_000_000u32;
        // scalar
        let t = Instant::now();
        for _ in 0..iters {
            filter_block2d_sixtap_safe(&src, stride, &mut dst, w, w, h, hf, vf);
            std::hint::black_box(&dst);
        }
        let scalar = t.elapsed().as_secs_f64() / iters as f64 * 1e9;
        // neon
        let t = Instant::now();
        for _ in 0..iters {
            filter_block2d_sixtap_neon(&src, stride, &mut dst, w, w, h, hf, vf);
            std::hint::black_box(&dst);
        }
        let neon = t.elapsed().as_secs_f64() / iters as f64 * 1e9;
        println!(
            "sixtap 16x16: scalar {scalar:.1} ns, neon {neon:.1} ns, speedup {:.2}x",
            scalar / neon
        );
    }
}
