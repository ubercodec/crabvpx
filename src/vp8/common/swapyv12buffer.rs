//! Reference-buffer swap — port of `vp8/common/swapyv12buffer.c`.
//!
//! One helper used during reference-frame management to exchange two YV12
//! frame buffers (e.g. rolling the just-decoded frame into the last-frame slot).

pub use crate::vp8::common::types::*;
pub type vpx_color_space = u32;

pub const VPX_CS_SRGB: vpx_color_space = 7;
pub const VPX_CS_RESERVED: vpx_color_space = 6;
pub const VPX_CS_BT_2020: vpx_color_space = 5;
pub const VPX_CS_SMPTE_240: vpx_color_space = 4;
pub const VPX_CS_SMPTE_170: vpx_color_space = 3;
pub const VPX_CS_BT_709: vpx_color_space = 2;
pub const VPX_CS_BT_601: vpx_color_space = 1;
pub const VPX_CS_UNKNOWN: vpx_color_space = 0;
pub type vpx_color_space_t = vpx_color_space;
pub type vpx_color_range = u32;
pub const VPX_CR_FULL_RANGE: vpx_color_range = 1;
pub const VPX_CR_STUDIO_RANGE: vpx_color_range = 0;
pub type vpx_color_range_t = vpx_color_range;
/// `vp8_swap_yv12_buffer` — vp8/common/swapyv12buffer.c:13.
///
/// Swaps the alloc and the three plane pointers between two YV12 buffers. C
/// swaps raw pointers; here it swaps the owning fields, same net effect.
pub fn vp8_swap_yv12_buffer_safe(
    new_frame: &mut YV12_BUFFER_CONFIG,
    last_frame: &mut YV12_BUFFER_CONFIG,
) {
    core::mem::swap(&mut new_frame.buffer_alloc, &mut last_frame.buffer_alloc);
    core::mem::swap(&mut new_frame.y_buffer, &mut last_frame.y_buffer);
    core::mem::swap(&mut new_frame.u_buffer, &mut last_frame.u_buffer);
    core::mem::swap(&mut new_frame.v_buffer, &mut last_frame.v_buffer);
}
