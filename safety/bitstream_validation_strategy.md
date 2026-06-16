# Bitstream Validation Strategy

This document details the strategy for hardened input validation and error handling in the VP8 decoder, ensuring resilience against malformed or malicious bitstreams.

## Principles of Hardened Validation

1.  **Fail-Fast and Graceful Recovery**: Any bitstream inconsistency must be detected as early as possible and result in a controlled error (`Result::Err`) rather than a panic or undefined behavior.
2.  **Exhaustive Pattern Matching**: Leverage Rust's `enum` and `match` constructs to handle all possible bitstream states, ensuring no "impossible" case is left unhandled.
3.  **Strict Bounds Checking**: Bitstream parsing is performed using slices and a dedicated reader that enforces size constraints.

## Validation Layers

### 1. Header Validation
Before any frame decoding begins, the frame header must be validated.
- **Size Checks**: Ensure the frame dimensions do not exceed the decoder's capabilities or allocated buffer sizes [vp8/decoder/decodeframe.rs:60].
- **Reserved Bits**: Check that reserved bits in the bitstream are set to expected values, returning `DecoderError::InvalidHeader` if they are not.

### 2. Boolean Decoder Integrity (`BOOL_DECODER`)
The boolean decoder [vp8/decoder/dboolhuff.rs:14] must be resilient to truncated bitstreams.
- **Underflow Detection**: The `vp8dx_bool_decoder_fill` function [vp8/decoder/dboolhuff.rs:53] and subsequent `read_bool` operations must check if the `user_buffer` has been exhausted.
- **Safe Fallback**: If a bit is requested but the buffer is empty, the decoder should return an error or a deterministic value (e.g., all zeros) while marking the frame as `corrupted`.

### 3. Entropy and Motion Vector Validation
- **Range Constraints**: Decoded values for motion vectors and transform coefficients must be checked against the limits defined in the VP8 specification [Filename: [Section 9](https://datatracker.ietf.org/doc/html/rfc6386#section-9)].
- **Index Validation**: When using decoded values as indices into tables (e.g., dequantization matrices), bounds checks are performed. Rust's indexing `table[index]` provides this automatically, but we prefer `table.get(index)` or pre-validation for better error reporting.

## Error Propagation and Recovery

The decoder will use a hierarchical error type:

```rust
pub enum DecoderError {
    BitstreamError(String),
    ResourceError(String),
    InvalidConfiguration(String),
    CorruptedFrame,
}
```

- **Recoverable Errors**: If a single macroblock is corrupted, the decoder may attempt to mask the error and continue with the next macroblock if the "error resilience" flag is set.
- **Unrecoverable Errors**: Errors in the uncompressed header or major bitstream synchronization issues will stop decoding of the current frame and return `Err(DecoderError::BitstreamError)`.

## Comparison with Legacy C
The original C code often sets a `corrupted` flag in the `YV12_BUFFER_CONFIG` [vpx_scale/generic/yv12config.rs:58]. While this flag is preserved in the Rust implementation for compatibility, the primary mechanism for error handling is the `Result` type, which prevents the application from accidentally using the output of a failed decoding operation.
