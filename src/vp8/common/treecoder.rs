pub use crate::vp8::common::types::vp8_prob;
pub type vp8_tree_index = ::core::ffi::c_schar;
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct vp8_token_struct {
    pub value: ::core::ffi::c_int,
    pub Len: ::core::ffi::c_int,
}
pub type vp8_token = vp8_token_struct;
pub type uint64_t = u64;
pub const vp8_prob_half: vp8_prob = 128 as ::core::ffi::c_int as vp8_prob;
