unsafe extern "Rust" {
    fn getenv(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strtol(
        __str: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: i32,
    ) -> ::core::ffi::c_long;
    fn sysctlbyname(
        _: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_void,
        oldlenp: *mut size_t,
        _: *mut ::core::ffi::c_void,
        newlen: size_t,
    ) -> i32;
}
pub type __darwin_size_t = usize;
pub type int64_t = i64;
pub type size_t = __darwin_size_t;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const HAS_NEON: i32 = (1 as i32) << 0 as i32;
pub const HAS_NEON_DOTPROD: i32 =
    (1 as i32) << 1 as i32;
pub const HAS_NEON_I8MM: i32 = (1 as i32) << 2 as i32;
pub const HAS_SVE: i32 = (1 as i32) << 3 as i32;
pub const HAS_SVE2: i32 = (1 as i32) << 4 as i32;
#[inline]
unsafe fn arm_cpu_env_flags(mut flags: *mut i32) -> i32 { unsafe {
    let mut env: *const ::core::ffi::c_char =
        getenv(b"VPX_SIMD_CAPS\0" as *const u8 as *const ::core::ffi::c_char);
    if !env.is_null() && *env as i32 != 0 {
        *flags = strtol(
            env,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            0 as i32,
        ) as i32;
        return 1 as i32;
    }
    return 0 as i32;
}}
#[inline]
unsafe fn arm_cpu_env_mask() -> i32 { unsafe {
    let mut env: *const ::core::ffi::c_char =
        getenv(b"VPX_SIMD_CAPS_MASK\0" as *const u8 as *const ::core::ffi::c_char);
    return if !env.is_null() && *env as i32 != 0 {
        strtol(
            env,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            0 as i32,
        ) as i32
    } else {
        !(0 as i32)
    };
}}
#[inline]
unsafe fn have_feature(mut feature: *const ::core::ffi::c_char) -> int64_t { unsafe {
    let mut feature_present: int64_t = 0 as int64_t;
    let mut size: size_t = ::core::mem::size_of::<int64_t>() as size_t;
    if sysctlbyname(
        feature,
        &raw mut feature_present as *mut ::core::ffi::c_void,
        &raw mut size,
        NULL,
        0 as size_t,
    ) != 0 as i32
    {
        return 0 as int64_t;
    }
    return feature_present;
}}
unsafe fn arm_get_cpu_caps() -> i32 { unsafe {
    let mut flags: i32 = 0 as i32;
    flags |= HAS_NEON;
    if have_feature(b"hw.optional.arm.FEAT_DotProd\0" as *const u8 as *const ::core::ffi::c_char)
        != 0
    {
        flags |= HAS_NEON_DOTPROD;
    }
    if have_feature(b"hw.optional.arm.FEAT_I8MM\0" as *const u8 as *const ::core::ffi::c_char) != 0
    {
        flags |= HAS_NEON_I8MM;
    }
    return flags;
}}
#[unsafe(no_mangle)]
pub unsafe fn arm_cpu_caps() -> i32 { unsafe {
    let mut flags: i32 = 0 as i32;
    if arm_cpu_env_flags(&raw mut flags) == 0 {
        flags = arm_get_cpu_caps() & arm_cpu_env_mask();
    }
    if flags & HAS_NEON_DOTPROD == 0 {
        flags &= !HAS_NEON_I8MM;
    }
    if flags & HAS_NEON_DOTPROD == 0 {
        flags &= !HAS_SVE;
    }
    if flags & HAS_NEON_I8MM == 0 {
        flags &= !HAS_SVE;
    }
    if flags & HAS_SVE == 0 {
        flags &= !HAS_SVE2;
    }
    return flags;
}}
