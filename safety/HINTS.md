# VP8 Decoder Safety Hints

## Current Progress (May 2026)
- **Build Fixes**: Modified `build.rs` to conditionalize ARM assembly files based on target architecture. `crabvpx` now successfully compiles on x86_64 (library only).
- **Boolean Decoder**: Implemented `SafeBoolDecoder<'a>` in `src/vp8/decoder/dboolhuff.rs` following `zero_unsafe_strategy.md`. Verified in isolation via `cargo +nightly test --test bool_decoder_test`.
- **Global State**: Converted `vp8_norm` in `src/vp8/common/entropy.rs` from `static mut` to immutable `static`.
- **SafeBoolDecoder Integration**: Removed duplicated `vp8dx_decode_bool` from `detokenize.rs`, `decodeframe.rs`, and `decodemv.rs`. Implemented a centralized `vp8dx_decode_bool` in `dboolhuff.rs` that delegates to `SafeBoolDecoder`.
- **x86_64 Testing Harness**: Implemented `simd_shim.rs` to forward missing `_neon` symbols to their `_c` equivalents on non-ARM targets. Also added a `pthread_once` shim in `thread_shim.rs` to fix Linux glibc compatibility issues with Darwin-transpiled `pthread_once_t` locks. Integration test harness `just compare` now runs and passes 35/35 vectors.
- **Frame Memory Management**: Refactored `YV12_BUFFER_CONFIG` memory allocation in `src/vpx_scale/generic/yv12config.rs` to use aligned Rust vectors (`Vec<Align32>`) instead of legacy `vpx_memalign`/`vpx_free`.
- **Safe Memory Allocation**: Replaced legacy `vpx_memalign` and `vpx_free` calls with `Box::try_new_zeroed()` and `Box::from_raw()` for `VP8D_COMP` allocation in `src/vp8/decoder/onyxd_if.rs`.

## Architectural Quirks to Watch Out For
- **c2rust Duplication**: Functions that were `static inline` in C headers (specifically `vp8dx_decode_bool` from `dboolhuff.h`) were duplicated by `c2rust` into every Rust module that called them. (Resolved for `vp8dx_decode_bool`).
- **Hardcoded NEON Calls**: The transpiled Rust code explicitly calls `_neon` function names (e.g., `vp8_bilinear_predict16x16_neon` in `decodeframe.rs`), because it was transpiled from C code where NEON macros were active. This causes link errors on x86_64 when building the integration `harness`.
- **Duplicated Structs**: Struct definitions like `YV12_BUFFER_CONFIG` and `VP8Common` were duplicated by `c2rust` across dozens of files. Do not attempt to deduplicate them yet; maintain raw pointer boundaries between modules to avoid FFI type mismatches.

## Next Steps for Future Agents
1. **Safe Memory Allocation**: Continue replacing `vpx_memalign` and `vpx_calloc` calls with safe Rust allocations (`Vec`, `Box`).
   - **Target 1**: `vpx_image_t` buffer allocation in `src/vpx/src/vpx_image.rs`.
2. **Public API Boundary**: Begin implementing Step 1 of `docs/refactor_plan.md` by creating a safe Rust `Decoder` wrapper around `vpx_codec_ctx_t`.

