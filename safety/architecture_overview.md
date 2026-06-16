# VP8 Decoder Rust Conversion: Architectural Design

This document provides a cohesive overview of the design for converting the VP8 video decoder from C to memory-safe Rust.

## Core Objective

The primary goal is to eliminate all `unsafe` blocks by replacing C-style pointer arithmetic and manual memory management with idiomatic Rust constructs like slices, smart pointers (`Arc`, `Box`), and robust error handling (`Result`).

## Design Components

The conversion strategy is divided into four main pillars:

### 1. Zero-Unsafe Strategy
We replace raw pointers with slices and encapsulate global state into structured objects. 
- **Detail**: [zero_unsafe_strategy.md](zero_unsafe_strategy.md)
- **Logic**: Slices provide built-in bounds checking, and structured objects ensure compliance with Rust's borrowing rules.

### 2. Targeted Pattern Mapping
We map specific unsafe C patterns (e.g., bitstream parsing, block indexing) to their safe Rust equivalents.
- **Detail**: [pattern_mapping.md](pattern_mapping.md)
- **Logic**: This provides a direct translation guide for implementors to follow when rewriting specific modules.

### 3. Frame Memory Management
We define a deterministic lifecycle for video frame buffers using a pool-based approach and reference counting.
- **Detail**: [frame_memory_management.md](frame_memory_management.md)
- **Logic**: This solves the complex problem of sharing decoded frames between the decoder (for reference) and the application (for display) without data races or use-after-free errors.

### 4. Hardened Input Validation
We implement a fail-fast validation strategy for bitstream parsing.
- **Detail**: [bitstream_validation_strategy.md](bitstream_validation_strategy.md)
- **Logic**: By using Rust's `Result` type and exhaustive pattern matching, we ensure the decoder is resilient against malformed or malicious input.

## Implementation Path

1.  **Foundation**: Implement the `FramePool` and `BitReader` abstractions.
2.  **Core Logic**: Translate the boolean decoder and entropy parsing using the `BitReader`.
3.  **Reconstruction**: Port the inverse transform and motion compensation logic, replacing pointer offsets with slice indexing.
4.  **Integration**: Implement the top-level decoder loop, managing the `Arc<FrameBuffer>` lifecycle.
5.  **Validation**: Test with the VP8 test suite (Conformance bitstreams) to ensure bit-identical output.

## Safety and Performance

By leveraging zero-copy mechanisms (like moving `Box` to `Arc`) and minimizing runtime overhead through efficient slice usage, this design achieves memory safety without sacrificing the performance required for video decoding.
