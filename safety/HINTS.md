# VP8 Decoder Safety Hints

## Current Progress (May 2026)
- **Build Fixes**: Modified `build.rs` to conditionalize ARM assembly files based on target architecture. `crabvpx` now successfully compiles on x86_64 (library only).
- **Boolean Decoder**: Implemented `SafeBoolDecoder<'a>` in `src/vp8/decoder/dboolhuff.rs` following `zero_unsafe_strategy.md`. Verified in isolation via `cargo +nightly test --test bool_decoder_test`.
- **Global State**: Converted `vp8_norm` in `src/vp8/common/entropy.rs` from `static mut` to immutable `static`.
- **SafeBoolDecoder Integration**: Removed duplicated `vp8dx_decode_bool` from `detokenize.rs`, `decodeframe.rs`, and `decodemv.rs`. Implemented a centralized `vp8dx_decode_bool` in `dboolhuff.rs` that delegates to `SafeBoolDecoder`.

## Architectural Quirks to Watch Out For
- **c2rust Duplication**: Functions that were `static inline` in C headers (specifically `vp8dx_decode_bool` from `dboolhuff.h`) were duplicated by `c2rust` into every Rust module that called them. (Resolved for `vp8dx_decode_bool`).
- **Hardcoded NEON Calls**: The transpiled Rust code explicitly calls `_neon` function names (e.g., `vp8_bilinear_predict16x16_neon` in `decodeframe.rs`), because it was transpiled from C code where NEON macros were active. This causes link errors on x86_64 when building the integration `harness`.

## Next Steps for Future Agents
1. **x86_64 Testing Harness**: To get `cd harness && cargo build --features rust` working on x86_64, implement a `simd_shim.rs` module that exports dummy `_neon` functions forwarding to the existing `_c` implementations (e.g., `vp8_bilinear_predict16x16_c` in `filter.rs`).
2. **Frame Memory Management**: Begin porting `YV12_BUFFER_CONFIG` in `yv12config.rs` to safe Rust vectors as outlined in `zero_unsafe_strategy.md`.
