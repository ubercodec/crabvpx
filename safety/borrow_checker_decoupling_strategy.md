# Borrow Checker Decoupling & "RefCell" Avoidance Strategy

## 1. The Looming Trap: The Legacy `Copy` Struct Problem

In the transpiled C codebase, `YV12_BUFFER_CONFIG` (which represents a video frame buffer) is a thin wrapper around raw pointers (`*mut u8`). Because pointers are cheap and ignore aliasing rules, the legacy decoder frequently performs operations like:

```rust
// Inside decodeframe.rs
pbi.mb.pre = pbi.common.yv12_fb[ref_fb_idx];
pbi.mb.dst = pbi.common.yv12_fb[new_fb_idx];
```

In Rust, if we attempt to make `YV12_BUFFER_CONFIG` safe by replacing raw pointers with safe owned buffers (e.g., `Vec<u8>`) or borrow slices (e.g., `&'a mut [u8]`), this pattern will immediately break the compiler:

1.  **If it owns the data (`Vec<u8>`)**: Assigning it will trigger massive, slow deep-copies of video frames, destroying performance.
2.  **If it references the data (`&'a mut [u8]`)**: 
    *   `MACROBLOCKD` (`mb`) will become lifetime-bound (`macroblockd<'a>`), infecting the entire call stack with complex lifetime parameters.
    *   The static borrow checker will reject borrowing `pre` (immutable reference frame) and `dst` (mutable destination frame) at the same time if it cannot prove they are disjoint, especially since they are elements of the same array (`yv12_fb`).

To bypass this, developers are often tempted to use **interior mutability (`RefCell` / `RwLock`)** or **unsafe raw pointer casting** to "quiet" the borrow checker. **This is a trap that introduces runtime overhead, risk of panics, and violates our safety goals.**

---

## 2. The Solution: Index-Based Referencing & Explicit Slice Passing

To achieve 100% safe, high-performance Rust without `RefCell`, we must decouple **buffer ownership** from the **decoder state structures**.

```
Legacy Dynamic Copying (Unsafe & Prone to Aliasing):
[Common Pool: yv12_fb] ──> (Copy Pointer Struct) ──> [MACROBLOCKD (xd)] ──> [Leaf Functions]
                                                         (xd.dst / xd.pre)

Safe Explicit Dataflow (Statically Checked & Zero Copy):
[Common Pool: yv12_fb] ──┐
                         ├─> [Orchestrator: decode_mb_row] ──> (Extract Slices) ──> [Leaf Functions]
[MACROBLOCKD (Indices)] ─┘                                                             (Explicit Slices)
```

### Principle 1: `MACROBLOCKD` Stores Indices, Not Buffers
We redefine `macroblockd` to store the metadata and the **index** of the frame buffer it needs, rather than holding the buffer itself.

```rust
// In src/vp8/common/types.rs
pub struct macroblockd {
    // ...
    pub dst_fb_idx: usize, // Index into pbi.common.yv12_fb
    pub pre_fb_idx: usize, // Index into pbi.common.yv12_fb
    pub dst_stride: i32,
    pub pre_stride: i32,
    // ...
}
```
Since `usize` implements `Copy`, we preserve the cheap assignment semantics of the C code without carrying any lifetime bindings.

### Principle 2: Extract Slices at the Orchestration Boundary
Instead of leaf functions retrieving buffers from `xd.dst` or `xd.pre`, the outermost orchestration function (which has access to both the global buffer pool `yv12_fb` and the macroblock context `xd`) extracts the slices and passes them down **explicitly**.

```rust
// In src/vp8/decoder/decodeframe.rs (Orchestrator)
let new_fb_idx = pbi.common.new_fb_idx as usize;
let ref_fb_idx = pbi.mb.pre_fb_idx;

// 1. Statically borrow the destination frame mutably
let (dst_y, dst_u, dst_v) = pbi.common.yv12_fb[new_fb_idx].views_mut();

// 2. Statically borrow the reference frame immutably (if applicable)
let (ref_y, ref_u, ref_v) = if has_ref {
    let ref_fb = &pbi.common.yv12_fb[ref_fb_idx];
    (Some(ref_fb.y_slice_safe()), Some(ref_fb.u_slice_safe()), Some(ref_fb.v_slice_safe()))
} else {
    (None, None, None)
};

// 3. Pass slices explicitly to reconstruction / prediction functions
vp8_decode_macroblock(
    &mut pbi.mb,
    dst_y, dst_u, dst_v,
    ref_y, ref_u, ref_v
);
```

This guarantees:
*   **Zero Runtime Overhead**: The borrow checker validates everything at compile time.
*   **Statically Checked Aliasing**: The compiler guarantees we never write to a buffer while reading it.
*   **No Lifetimes on Structs**: The core structures remain clean and easy to refactor.

---

## 3. Backlog of Refactoring Work Items

Before removing more `unsafe` blocks from the reconstruction and filtering modules, execute the following decoupling steps to prevent getting backed into a borrowing corner.

### Milestone 1: Decouple `macroblockd` from `YV12_BUFFER_CONFIG` (Structural Decoupling)
Goal: Eliminate the direct inclusion of `YV12_BUFFER_CONFIG` inside `macroblockd` to prevent lifetime infection.

*   **[x] Unit 1 (`types.rs`):** Replace `pub pre: YV12_BUFFER_CONFIG` and `pub dst: YV12_BUFFER_CONFIG` in `struct macroblockd` with index fields (`pre_fb_idx: usize`, `dst_fb_idx: usize`) and metadata fields (strides, dimensions).
*   **[x] Unit 2 (`decodeframe.rs`):** Update call sites that perform `pbi.mb.pre = ...` to instead copy the corresponding indices:
    ```rust
    pbi.mb.dst_fb_idx = new_fb_idx;
    pbi.mb.pre_fb_idx = ref_fb_idx;
    ```
*   **[x] Unit 3 (`threading.rs`):** Apply the same index assignment refactor to the multithreaded row decoder initialization.

### Milestone 2: Refactor Inter-Prediction & Motion Compensation (Explicit Dataflow)
Goal: Stop passing `*mut macroblockd` to prediction functions; pass slices explicitly.

*   **[x] Unit 4 (`reconinter.rs`):** Refactor inter-prediction helpers (e.g., `vp8_build_inter_predictors`) to accept input/output slices explicitly:
    ```rust
    pub fn vp8_build_inter_predictors_safe(
        xd: &macroblockd,
        ref_y: &[u8], ref_u: &[u8], ref_v: &[u8],
        dst_y: &mut [u8], dst_u: &mut [u8], dst_v: &mut [u8],
    )
    ```
*   **[x] Unit 5 (`decodeframe.rs` & `threading.rs`):** Update macroblock decoding pipelines to extract slices using `views_mut()` at the row loop level, and pass them to the refactored inter-prediction helpers.

### Milestone 3: Refactor Loop Filtering Slicing (Disjoint Borrows)
Goal: Perform loop filtering in-place on disjoint slices without locking the entire frame structure.

*   **[ ] Unit 6 (`loopfilter.rs`):** Implement a `split_rows_mut` or safe chunking method on `YV12_BUFFER_CONFIG` that yields disjoint mutable slices for individual macroblock rows.
*   **[ ] Unit 7 (`threading.rs`):** Refactor multithreaded loop filtering to assign each thread a strictly disjoint mutable slice of the frame, proving to the borrow checker that parallel loop filtering is 100% race-free.

---

## 4. Rules of Engagement for Refactoring Agents

1.  **Refuse Interior Mutability:** Under no circumstances should `RefCell`, `std::cell::Cell`, `Mutex` (outside threading synchronization), or unsafe pointer casts be introduced to bypass borrowing errors.
2.  **Fail Fast on Borrow Checker Conflicts:** If the borrow checker rejects a safe implementation, do not try to patch it with `unsafe`. Stop, analyze the aliasing pattern, and refactor the caller to pass disjoint slices explicitly.
3.  **Bit-Exact Verification:** Run `./build.sh` and `./scripts/compare.py` after every refactoring unit to guarantee that changing the dataflow did not alter the decoder's output.
