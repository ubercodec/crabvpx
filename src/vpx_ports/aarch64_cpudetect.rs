pub const HAS_NEON: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const HAS_NEON_DOTPROD: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const HAS_NEON_I8MM: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const HAS_SVE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const HAS_SVE2: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;

fn parse_int_base_0(s: &str) -> Option<i32> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    let (s, negative) = if s.starts_with('-') {
        (&s[1..], true)
    } else if s.starts_with('+') {
        (&s[1..], false)
    } else {
        (s, false)
    };

    let parsed = if s.starts_with("0x") || s.starts_with("0X") {
        let hex_str = s.trim_start_matches("0x").trim_start_matches("0X");
        i32::from_str_radix(hex_str, 16).ok()
    } else if s.starts_with('0') && s.len() > 1 {
        let oct_str = s.trim_start_matches('0');
        if oct_str.is_empty() {
            Some(0)
        } else {
            i32::from_str_radix(oct_str, 8).ok()
        }
    } else {
        s.parse::<i32>().ok()
    };

    parsed.map(|val| if negative { -val } else { val })
}

fn arm_cpu_env_flags() -> Option<i32> {
    if let Ok(val) = std::env::var("VPX_SIMD_CAPS") {
        parse_int_base_0(&val)
    } else {
        None
    }
}

fn arm_cpu_env_mask() -> i32 {
    if let Ok(val) = std::env::var("VPX_SIMD_CAPS_MASK") {
        parse_int_base_0(&val).unwrap_or(!0)
    } else {
        !0
    }
}

fn have_feature(_feature: &str) -> i64 {
    // TODO: Use getauxval on Linux
    0
}

fn arm_get_cpu_caps() -> ::core::ffi::c_int {
    let mut flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    flags |= HAS_NEON;
    if have_feature("hw.optional.arm.FEAT_DotProd") != 0 {
        flags |= HAS_NEON_DOTPROD;
    }
    if have_feature("hw.optional.arm.FEAT_I8MM") != 0 {
        flags |= HAS_NEON_I8MM;
    }
    flags
}

pub fn arm_cpu_caps() -> ::core::ffi::c_int {
    let mut flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if let Some(env_flags) = arm_cpu_env_flags() {
        flags = env_flags;
    } else {
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
    flags
}
