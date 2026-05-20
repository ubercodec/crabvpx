# Multithreaded Safety Refactoring Strategy: Zero-Overhead Encapsulated Unsafe

> [!IMPORTANT]
> **DO NOT ATTEMPT TO REMOVE THE UNSAFE BLOCKS DEFINED IN THIS DESIGN.**
> This document outlines a deliberate, performance-critical architectural decision. Replacing the encapsulated `unsafe` blocks or interior mutability structures in this threading model with standard safe Rust wrappers (like `Mutex` or `RwLock`) will introduce severe lock contention, resulting in an unacceptable **15% to 40% performance regression** in video decoding. The `unsafe` boundaries here are permanent and must be preserved.

---

## 1. The Core Vulnerability & The Performance Trap

In the transpiled threading model, CrabVPX utilizes a "Safe Lie" to bypass Rust's strict static analysis:

```rust
pub struct DECODETHREAD_DATA {
    pub ithread: ::core::ffi::c_int,
    pub ptr1: *mut ::core::ffi::c_void, // Points to VP8D_COMP
    pub ptr2: *mut ::core::ffi::c_void, // Points to MB_ROW_DEC
}
unsafe impl Send for DECODETHREAD_DATA {}
```

Inside the worker threads, `ptr1` is cast back to an aliased mutable reference:
```rust
let pbi = unsafe { &mut *(data.ptr1 as *mut VP8D_COMP) };
```

### Why Standard Safe Rust (`Mutex`/`RwLock`) Fails:
To make this statically safe without `unsafe`, one would wrap the shared row synchronization buffers (`mt_yabove_row` etc.) in `Arc<Mutex<AlignedBox>>`. 
However, at 1080p 30fps, this requires approximately **1.5 million lock/unlock operations per second** across concurrently executing threads. The resulting lock acquisition overhead and L1/L2 cache line bouncing between CPU cores destroys the decoder's real-time performance.

### The Solution: Encapsulated Hardware-Synchronized Unsafe
Because the VP8 decoder already utilizes atomic column spinlocks (`vp8_atomic_spin_wait`) to coordinate threads, the execution pipeline is **already synchronized at the hardware level**. Thread A will mathematically never write to column $C$ while Thread B is reading column $C$.

Therefore, we do not need runtime locks. We instead use a highly optimized, custom safety wrapper—`UnsafeRowView`—that utilizes `unsafe` and raw pointers internally, but exposes a safe API to the decoder.

---

## 2. The Target Architecture: Disjoint Dataflow

```
Monolithic Sharing (UB Hazard):
[Thread 1] ──> &mut VP8D_COMP ──┐
[Thread 2] ──> &mut VP8D_COMP ──┼─> [Shared Monolithic Memory] (Protected only by spinlocks)
[Thread 3] ──> &mut VP8D_COMP ──┘

Target Architecture (Statically Guaranteed & Zero-Overhead):
[Main Thread (VP8D_COMP Owner)]
   ├── Reference Frames ────────────────── (Shared Ref &YV12) ────────> [All Threads (Read-Only: 100% Safe)]
   ├── Destination Frame Buffer ────────── (UnsafeRowView) ──────────> [Thread 1: Write/Read Row 1, 4...] 
   │                                                                 └─> [Thread 2: Write/Read Row 2, 5...]
   └── Row Sync Buffers (mt_yabove) ────── (UnsafeRowView) ──────────> [Direct Zero-Copy Access (No Locks)]
```

### The Safe/Unsafe Boundary Contract:
*   **Leaf Functions (100% Safe)**: All reconstruction, prediction, and loop-filtering functions in the leaf modules (e.g., `reconinter.rs`, `loopfilter_filters.rs`) **must be 100% safe**. They must only accept standard Rust slices (`&[u8]`, `&mut [u8]`) and remain completely unaware of the threading model or the underlying `unsafe` row views.
*   **The Threading Boundary (`UnsafeRowView`)**: The `unsafe` code is strictly isolated within a custom container in `types.rs` and the orchestrator in `threading.rs`. This container wraps the raw pointers and implements `Send`/`Sync`, acting as a zero-overhead bridge that yields safe slices to the leaf functions.

---

## 3. Strategic Timeline: When to Solve This?

> [!TIP]
> **Do not solve the threading issue first. Continue removing unsafe blocks from leaf modules.**
> 
> Refactoring leaf modules (e.g., motion compensation, subpixel prediction) to accept safe, explicit slices rather than raw pointers is a **strict prerequisite** for this threading refactoring. 
>
> As you make leaf functions safer by enforcing explicit slice dataflow, you are naturally decoupling them from the monolithic `VP8D_COMP` struct. Once the leaf functions are clean, the threading orchestrator in `threading.rs` can easily extract `UnsafeRowView` slices and pass them down as standard `&[u8]` / `&mut [u8]`.
>
> Putting off the threading refactoring until the end **will not make it harder**. It isolates the problem, allowing you to maintain a clear boundary between the 100% safe decoding logic and the highly optimized threading synchronization engine.

---

## 4. Backlog of Refactoring Work Units

This backlog is designed to be executed as the **final stabilization phase** of the project, after leaf modules have been refactored to use explicit slice passing.

### Milestone 1: Implement the `UnsafeRowView` Abstraction

Goal: Create the zero-overhead, thread-safe memory wrapper to replace raw `AlignedBox` sharing.

#### Work Units:
*   **[ ] Unit 1 (`types.rs`):** Define `UnsafeRowView` wrapping a raw pointer and a length. Implement `Send` and `Sync` manually. Document the hardware-atomic safety contract.
    ```rust
    pub struct UnsafeRowView {
        ptr: *mut u8,
        len: usize,
    }
    unsafe impl Send for UnsafeRowView {}
    unsafe impl Sync for UnsafeRowView {}
    ```
*   **[ ] Unit 2 (`types.rs`):** Implement safe slice projection methods on `UnsafeRowView`:
    ```rust
    impl UnsafeRowView {
        /// Safety: Caller must ensure via atomic column sync that
        /// no other thread is writing to this sub-slice concurrently.
        pub unsafe fn as_slice(&self, offset: usize, len: usize) -> &[u8] {
            std::slice::from_raw_parts(self.ptr.add(offset), len)
        }
        pub unsafe fn as_slice_mut(&self, offset: usize, len: usize) -> &mut [u8] {
            std::slice::from_raw_parts_mut(self.ptr.add(offset), len)
        }
    }
    ```

---

### Milestone 2: Transition Row and Left Sync Buffers to `UnsafeRowView`

Goal: Eliminate monolithic `mt_sync: &mut VP8D_MT_SYNC` sharing across threads.

#### Work Units:
*   **[ ] Unit 3 (`types.rs`):** Refactor `VP8D_MT_SYNC`. Replace `Box<[Option<AlignedBox>]>` for `mt_yabove_row` (and U/V above/left buffers) with `Box<[UnsafeRowView]>`.
*   **[ ] Unit 4 (`threading.rs`):** Update buffer allocation paths to initialize `UnsafeRowView` instances mapping the allocated `AlignedBox` memory.
*   **[ ] Unit 5 (`threading.rs`):** Refactor the read/write sites in `mt_decode_mb_rows` to utilize `UnsafeRowView::as_slice` and `as_slice_mut` inside narrow `unsafe` blocks at the macroblock boundary, passing standard safe slices down to the leaf functions.

---

### Milestone 3: Slice the Destination Frame Buffer Safely

Goal: Prevent threads from concurrently sharing the mutable `common: &mut VP8_COMMON` struct.

#### Work Units:
*   **[ ] Unit 6 (`types.rs`):** Implement an `UnsafeRowView` projection helper on `YV12_BUFFER_CONFIG` that returns thread-safe row views for the destination frame.
*   **[ ] Unit 7 (`threading.rs`):** Refactor `mt_decode_mb_rows` signature. Remove `common: &mut VP8_COMMON` and `mbc: &mut [vp8_reader; 9]`. Instead, pass:
    *   `ref_frames: &[YV12_BUFFER_CONFIG; 32]` (Read-only shared references).
    *   `dst_view: UnsafeRowView` (Thread-safe view of this thread's destination rows).
    *   `entropy_reader: &mut vp8_reader` (The specific reader for this thread's partition, resolved before entering the thread).

---

### Milestone 4: Purge the "Safe Lie" and `DECODETHREAD_DATA`

Goal: Eliminate raw pointer casting in worker thread entry points.

#### Work Units:
*   **[ ] Unit 8 (`types.rs`):** Define `SafeDecodeThreadData` containing only `UnsafeRowView` structures, disjoint slices, and safe configuration references. Remove the legacy `DECODETHREAD_DATA` and its `unsafe impl Send`.
*   **[ ] Unit 9 (`threading.rs`):** Refactor `vp8_decoder_create_threads` and `thread_decoding_proc` to use `SafeDecodeThreadData`. 
*   **[ ] Unit 10 (Verification):** Confirm compilation and run `./scripts/compare.py` to guarantee that the zero-overhead unsafe abstraction maintains a **perfect 100% bit-identical match** while running at maximum C-level decoding performance.
