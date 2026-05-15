# Zero-Unsafe Abstraction Strategy

This document outlines the architectural approach to achieving a zero-unsafe codebase for the VP8 decoder, replacing C-legacy pointer arithmetic with memory-safe Rust abstractions.

## Core Principles

1.  **Slice-Based Boundaries**: Replace all raw pointers used for buffer traversal with Rust slices (`&[u8]`, `&mut [u8]`). This ensures that bounds checks are enforced by the compiler and runtime.
2.  **Encapsulated State**: Move global and static mutable state into structured objects that adhere to Rust's borrowing rules.
3.  **Type-Safe Bitstream Parsing**: Abstract the bitstream into a dedicated reader that manages its own cursor and bounds, eliminating the risk of over-reads.
4.  **Safe SIMD Wrappers**: Encapsulate performance-critical SIMD code behind safe interfaces that validate input dimensions before execution.

## Key Transformations

### 1. Bitstream Reading (`BOOL_DECODER`)
The legacy `BOOL_DECODER` [vp8/decoder/dboolhuff.rs:14] uses raw pointers `user_buffer` and `user_buffer_end`. 

**Transformation**:
- Replace `user_buffer` and `user_buffer_end` with a single lifetime-bound slice `&'a [u8]`.
- Implement `vp8dx_start_decode` [vp8/decoder/dboolhuff.rs:28] as a constructor for a `SafeBoolDecoder<'a>` struct.
- Use an internal index (`usize`) instead of pointer offsets.
- Safety Argument: Using a slice inherently prevents reading past the end of the provided bitstream, as any attempt to index out of bounds will panic (or return an error in a checked context), rather than causing undefined behavior.

### 2. Frame Buffer Management (`YV12_BUFFER_CONFIG`)
The `YV12_BUFFER_CONFIG` [vpx_scale/generic/yv12config.rs:29] uses raw pointers for `y_buffer`, `u_buffer`, and `v_buffer` which are offsets into a single `buffer_alloc` [vpx_scale/generic/yv12config.rs:47].

**Transformation**:
- Replace raw pointers with a structured `Frame` type containing `Vec<u8>` or an `Arc<[u8]>`.
- Provide access to planes via slice methods (e.g., `fn y_plane(&self) -> &[u8]`).
- Use `split_at_mut` to provide non-overlapping mutable access to different planes of the same buffer, satisfying the borrow checker.
- Safety Argument: This eliminates the need for manual memory management in `vp8_yv12_de_alloc_frame_buffer` [vpx_scale/generic/yv12config.rs:65] and prevents use-after-free or double-free errors.

### 3. Block and Macroblock Context
The decoder often uses pointers to "above" and "left" contexts to inform prediction.

**Transformation**:
- Use a `ContextTracker` struct that owns or borrows the line buffers.
- Use safe indexing into these buffers.
- Replace pointer-swapping with a `swap()` method on indices or references.

## Handling SIMD and Assembly
Where SIMD is required for performance (e.g., `vp8_loop_filter_neon` [vp8/common/arm/neon/vp8_loopfilter_neon.rs:146]), we will:
1.  Keep the implementation in a private module.
2.  Provide a safe public wrapper that takes slices.
3.  Verify that `slice.len()` is sufficient for the SIMD operation's requirements (e.g., 16-byte alignment and minimum size).
4.  Only use `unsafe` within these strictly controlled wrappers, with a goal of eventually migrating to `core::simd` once stabilized.

## Error Handling
Instead of the `vpx_internal_error` pointer-based side-effect pattern, use `Result<T, DecoderError>`. This ensures that errors in the bitstream are handled upstream and cannot leave the decoder in an invalid, "unsafe" state.
