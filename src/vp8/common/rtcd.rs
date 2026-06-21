//! Runtime kernel dispatch init — port of `vp8/common/vp8_rtcd`.
//!
//! Idempotent, thread-safe one-time setup mirroring libvpx's `vp8_rtcd()`
//! (CPU-feature selection of kernel implementations).

use std::sync::Once;

static INIT: Once = Once::new();

fn setup_rtcd_internal() {
    // Pure-Rust generic build: no runtime SIMD dispatch to initialize.
}

pub fn vp8_rtcd() {
    INIT.call_once(|| {
        setup_rtcd_internal();
    });
}
