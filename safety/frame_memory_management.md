# Frame Memory Management and Ownership Lifecycle

This document defines the strategy for managing video frame buffers within the VP8 decoder, ensuring a leak-free and race-free lifecycle through deterministic ownership transitions and smart pointers.

## Overview

To replace the legacy C-style manual reference counting and raw pointer arithmetic, we adopt a reference-counted pool-based management system. This ensures that memory is never freed while still in use by the decoder or the consumer, and that data races are prevented by Rust's ownership and borrowing rules.

## The Ownership Contract

The lifecycle of a `FrameBuffer` is governed by a transition through three states: **Owned**, **Shared (Reference)**, and **Shared (Consumer)**.

| Phase | Ownership Transition | Choice of Smart Pointer | Safety Argument |
| :--- | :--- | :--- | :--- |
| **Acquisition & Decoding** | `FramePool` &rarr; `Decoder` | `Box<FrameBuffer>` (Unique) | The pool provides a unique, owned buffer. The decoder has exclusive mutable access during reconstruction, preventing concurrent access. Replaces unsafe `buffer_alloc` [vpx_scale/generic/yv12config.rs:47]. |
| **Reference Promotion** | `Decoder` &rarr; `Reference Pool` | `Arc<FrameBuffer>` | Once decoding is complete, the `Box` is converted to a shared `Arc`. This is a zero-copy "freeze" operation. The decoder stores this in the `ReferencePool` (Last, Golden, or AltRef slots). |
| **Frame Handover** | `Reference Pool` &rarr; `Consumer` | `Arc<FrameBuffer>` | For "show frames," the decoder clones the `Arc` and passes it to the consumer. Memory remains valid even if the decoder replaces the frame in its own reference slots. Replaces unsafe pointer `frame_to_show` [vp8/common/onyxc_int.h:76]. |
| **Slot Retirement** | `Reference Pool` &rarr; `FramePool` | `Arc<FrameBuffer>` (Drop) | When the decoder updates a reference slot, the old `Arc` is dropped. If the consumer has already dropped its clone, the reference count hits zero. Replaces manual `fb_idx_ref_cnt` decrements [vp8/common/onyxc_int.h:79]. |
| **Display Completion** | `Consumer` &rarr; `FramePool` | `Arc<FrameBuffer>` (Drop) | When the consumer (e.g., a renderer) is finished, it drops its `Arc`. If the decoder has also retired this frame, the buffer is returned to the pool for reuse. |

## Buffer Management Strategy

### 1. The Frame Pool
The `FramePool` is a thread-safe manager that maintains a set of pre-allocated `FrameBuffer` structures. 
- It uses a `crossbeam-channel` or a `Mutex<Vec<...>>` to track available buffers.
- It prevents memory fragmentation and reduces allocation overhead.
- Upon `Drop` of the final `Arc` reference, the buffer is automatically sent back to the pool.

### 2. Smart Pointer Selection: `Arc<FrameBuffer>`
- **Why not `Box`?** While `Box` is sufficient for the initial decoding phase, it cannot be shared between the reference pool and the consumer without unsafe cloning.
- **Why `Arc`?** Using `Arc` leverages Rust's built-in, atomic reference counting, which is less error-prone and automatically handles cleanup.
- **Safety Mechanism**: By providing only `&FrameBuffer` (read-only access) once the frame is promoted to the `Arc` phase, we satisfy the "Multiple Readers OR One Writer" rule.

## Safety Provenance

- **Zero-Copy**: The transition from `Box<FrameBuffer>` to `Arc<FrameBuffer>` is zero-copy.
- **No Use-After-Free**: The `Arc` ensures the underlying `FrameBuffer` is only returned to the pool when the absolute last reference is dropped.
- **No Data Races**: The transition from mutable (unique) to immutable (shared) is enforced by the type system.
- **Bounds Enforcement**: Access to pixels is provided through slices (`&[u8]`), preventing buffer overflows during motion compensation [vpx_scale/generic/yv12config.rs:43].
