# VP8 Decoder: Unsafe Pattern Mapping

This document identifies common unsafe patterns found in the legacy VP8 decoder and maps them to their idiomatic, memory-safe Rust alternatives.

## Targeted Modification List

| C/Unsafe Pattern | Safe Rust Construct |
| :--- | :--- |
| **Pointer-based block indexing and arithmetic** (e.g., `short *qcoeff`, `ptr.offset(i)`) | **Slices and Indexing**: Use `&[u8]` or `&mut [u16]` with standard indexing (which includes bounds checks) or `split_at_mut` for non-overlapping sub-slices. |
| **Manual bitstream parsing** (e.g., manual bit-shifting and masking from raw byte pointers) | **Bitstream Reader Abstraction**: Encapsulate the bitstream in a `BitReader` struct that provides safe methods like `read_bits(n)` and `read_bool()`, backed by a slice. Alternatively, use the `nom` crate for declarative parsing. |
| **Boolean/Arithmetic coding with raw pointers** (e.g., `BOOL_DECODER` using `user_buffer` and `user_buffer_end`) | **Safe Arithmetic Decoder**: Implement the boolean decoder using a slice `&[u8]` and a cursor/index. The logic for `range` and `value` remains the same but within a safe interface. |
| **Pointer-based macroblock/block context tracking** (e.g., `above_context` and `left_context` pointers) | **Structured Context Manager**: Store entropy contexts in a dedicated structure that manages the "above" and "left" neighbors using indexing into a fixed-size buffer or safe references. |
| **Manual memory management for frame buffers** (e.g., `y_buffer`, `u_buffer`, `v_buffer` in `YV12_BUFFER_CONFIG`) | **Owned Buffers (RAII)**: Use `Vec<u8>` or `Box<[u8]>` to ensure memory is automatically freed. Use a `FrameBuffer` struct to manage the three planes together. |
| **Reference frame ownership via pointers** (e.g., `dec_fb_ref` array of pointers) | **Reference Counting (`Arc`)**: Use `Option<Arc<FrameBuffer>>` to manage reference frames. This ensures that a frame remains valid as long as the decoder (or the consuming application) holds a reference to it. |
| **Global mutable state** (e.g., `static mut vp8_norm`, `static mut kCat3456`) | **Immutable Constants or Instance State**: Move read-only tables to `const` or `static` with no `mut`. Move mutable state into a `DecoderState` struct passed by reference to functions. |
| **Manual Threading and Synchronization** (e.g., `pthread_t`, `semaphore_t`, and raw pointers to shared state) | **Safe Concurrency Primitives**: Use `std::thread`, `std::sync::Mutex`, `std::sync::Condvar`, or higher-level abstractions like `Rayon` or `crossbeam` to safely manage shared state across threads. |
| **SIMD/Assembly calls with raw pointers** (e.g., NEON/SSE functions taking `*mut u8`) | **Safe SIMD Wrappers**: Wrap SIMD intrinsics in safe functions that take slices, ensuring that the slice length matches the expected SIMD block size before calling the unsafe intrinsic. |
| **Pointer-based Error Handling** (e.g., passing `vpx_internal_error_info *` pointers) | **Result Type**: Use Rust's `Result<T, DecoderError>` to propagate errors idiomatially, avoiding the need for side-effect-based error reporting. |

## Ownership and Lifecycle Strategy

A critical aspect of the transition to Rust is the ownership contract between the decoder and the application.

1.  **Buffer Allocation**: The application or a dedicated `BufferPool` provides empty buffers to the decoder.
2.  **Decoding**: The decoder populates a `FrameBuffer` (wrapped in `Arc`).
3.  **Output**: The decoder returns an `Arc<FrameBuffer>` to the application.
4.  **Reference Tracking**: The decoder keeps its own `Arc<FrameBuffer>` copies in its `dec_fb_ref` slots for future inter-frame prediction.
5.  **Reclamation**: When both the application and the decoder release their `Arc` references, the memory is automatically returned to the pool or freed.

This strategy replaces all manual pointer tracking and "corrupted" flag management in the original C code.
