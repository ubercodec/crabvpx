# CrabVPX

A **memory-safe, pure-Rust VP8 video decoder**. Originally lifted from `libvpx`
with `c2rust` and progressively rewritten into safe Rust.

- **Correct:** decode output is bit-exact with `libvpx` across all 62 WebM VP8
  conformance vectors — comprehensive, intra, inter, segmentation, partitions,
  sharpness, and smallsize — single- and multi-threaded (verified in CI).
- **Fast:** on **aarch64**, NEON kernels for the loop filter and sub-pixel
  filter bring decode to roughly **1.1–1.5× of `libvpx`** on heavy content
  and *faster* than libvpx on lighter streams. Other targets use the pure-Rust
  scalar path (LLVM auto-vectorized), ~1.5–3× of libvpx. All SIMD `unsafe`
  is confined to `vp8/common/neon.rs`, each kernel bit-exact-gated against
  its scalar twin.
- **Safe:** builds on stable Rust (no `unsafe`-heavy FFI in the decode path;
  the remaining `unsafe` is confined to the low-level buffer/threading core).
- **Scope:** VP8 *decoding* only (no encoder, no VP9/AV1).

## Usage

```rust
use crabvpx::api::{Decoder, Plane, Vp8Decoder};

let mut dec = Vp8Decoder::new();
dec.init()?;
dec.decode(frame)?; // one compressed VP8 frame
while let Some(img) = dec.get_frame()? {
    let y = img.plane(Plane::Y).unwrap();
    let stride = img.stride(Plane::Y);
    // ... use img.width(), img.height(), planes ...
}
```

Multithreaded decoding is opt-in via `CRABVPX_THREADS` (default `1`).

## Provenance

## Project Status Checklist

### Phase 1: Preparation & Configuration
- [x] Clone upstream `libvpx` repository.
- [x] Configure `libvpx` build strictly for VP8 decoding (e.g., `--disable-vp9`, `--disable-vp8-encoder`).
- [x] Generate `compile_commands.json` using `bear`.

### Phase 2: Transpilation (The Lift)
- [x] Run `c2rust transpile` on the compilation database.
- [x] Initialize `crabvpx` Cargo project.
- [x] Integrate generated `.rs` files into the Cargo structure.

### Phase 3: Hardware Intrinsics & Compilation (The Shift)
- [x] Verify transpiled NEON intrinsics compile purely in Rust using `std::arch::aarch64`.
- [x] Fix transpilation edge cases (atomics, `c_variadic`, types).
- [x] Achieve clean `cargo check` on Rust 2021.

### Phase 4: Stabilization & Upgrade
- [x] Port/integrate tests (harness) to verify decoding correctness against the full 62 IVF vectors.
- [x] Upgrade codebase to Rust 2024 using `cargo fix --edition`.
- [ ] Implement Phase 5: Incremental Refactoring to Safe Rust APIs (see `docs/refactor_plan.md`).
