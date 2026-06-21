//! Build configuration — port of the generated `vpx_config.h`.
//!
//! Compile-time feature/arch flags for the minimal vp8-decode-only build.

static cfg: &[u8] = b"--disable-examples --disable-tools --disable-docs --disable-unit-tests --disable-vp9 --disable-vp8-encoder\0";
#[unsafe(no_mangle)]
pub extern "C" fn vpx_codec_build_config() -> *const ::core::ffi::c_char {
    cfg.as_ptr() as *const ::core::ffi::c_char
}
