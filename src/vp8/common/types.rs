pub type uint32_t = u32;
pub type uint8_t = u8;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MV {
    pub row: ::core::ffi::c_short,
    pub col: ::core::ffi::c_short,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union int_mv {
    pub as_int: uint32_t,
    pub as_mv: MV,
}
