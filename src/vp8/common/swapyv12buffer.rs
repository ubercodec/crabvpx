use crate::vpx_scale::generic::yv12config::Yv12BufferConfig;

pub use crate::vpx::src::vpx_image::{
    VPX_CR_FULL_RANGE, VPX_CR_STUDIO_RANGE, VPX_CS_BT_601, VPX_CS_BT_709, VPX_CS_BT_2020,
    VPX_CS_RESERVED, VPX_CS_SMPTE_170, VPX_CS_SMPTE_240, VPX_CS_SRGB, VPX_CS_UNKNOWN,
};

/// Swaps the buffer pointers between two YV12 configurations.
///
/// This is typically used in the encoder/decoder to swap the current and last frame buffers.
///
/// # Safety
///
/// Both `new_frame` and `last_frame` must be valid, non-null pointers to `Yv12BufferConfig` structs.
/// The caller must ensure that the underlying memory is not accessed concurrently.
#[unsafe(no_mangle)]
pub unsafe fn vp8_swap_yv12_buffer(
    new_frame: *mut Yv12BufferConfig,
    last_frame: *mut Yv12BufferConfig,
) {
    if new_frame.is_null() || last_frame.is_null() {
        return;
    }

    // SAFETY: We checked for null above. The caller must ensure validity.
    let new_frame = unsafe { &mut *new_frame };
    let last_frame = unsafe { &mut *last_frame };

    last_frame.swap_buffers(new_frame);
}
