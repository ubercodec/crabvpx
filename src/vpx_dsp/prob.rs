pub type int8_t = i8;
pub type uint8_t = u8;
pub type uint64_t = u64;
pub type vpx_prob = uint8_t;
pub type vpx_tree_index = int8_t;
#[inline]
unsafe fn get_prob(
    mut num: ::core::ffi::c_uint,
    mut den: ::core::ffi::c_uint,
) -> vpx_prob {
    let p: i32 = (num as uint64_t)
        .wrapping_mul(256 as uint64_t)
        .wrapping_add((den >> 1 as i32) as uint64_t)
        .wrapping_div(den as uint64_t) as i32;
    let clipped_prob: i32 = p
        | (255 as i32 - p) >> 23 as i32
        | (p == 0 as i32) as i32;
    clipped_prob as vpx_prob
}
#[inline]
unsafe fn weighted_prob(
    mut prob1: i32,
    mut prob2: i32,
    mut factor: i32,
) -> vpx_prob {
    ((prob1 * (256 as i32 - factor)
        + prob2 * factor
        + ((1 as i32) << (8 as i32 - 1 as i32)))
        >> 8 as i32) as vpx_prob
}
static mut count_to_update_factor: [i32; 21] = [
    0 as i32,
    6 as i32,
    12 as i32,
    19 as i32,
    25 as i32,
    32 as i32,
    38 as i32,
    44 as i32,
    51 as i32,
    57 as i32,
    64 as i32,
    70 as i32,
    76 as i32,
    83 as i32,
    89 as i32,
    96 as i32,
    102 as i32,
    108 as i32,
    115 as i32,
    121 as i32,
    128 as i32,
];
#[inline]
unsafe fn mode_mv_merge_probs(
    mut pre_prob: vpx_prob,
    mut ct: *const ::core::ffi::c_uint,
) -> vpx_prob {
    unsafe {
        let den: ::core::ffi::c_uint = (*ct.offset(0 as i32 as isize))
            .wrapping_add(*ct.offset(1 as i32 as isize));
        if den == 0 as ::core::ffi::c_uint {
            pre_prob
        } else {
            let count: ::core::ffi::c_uint = if den < 20 as ::core::ffi::c_uint {
                den
            } else {
                20 as ::core::ffi::c_uint
            };
            let factor: ::core::ffi::c_uint =
                count_to_update_factor[count as usize] as ::core::ffi::c_uint;
            let prob: vpx_prob =
                get_prob(*ct.offset(0 as i32 as isize), den) as vpx_prob;
            weighted_prob(
                pre_prob as i32,
                prob as i32,
                factor as i32,
            )
        }
    }
}
#[unsafe(no_mangle)]
pub static mut vpx_norm: [uint8_t; 256] = [
    0 as i32 as uint8_t,
    7 as i32 as uint8_t,
    6 as i32 as uint8_t,
    6 as i32 as uint8_t,
    5 as i32 as uint8_t,
    5 as i32 as uint8_t,
    5 as i32 as uint8_t,
    5 as i32 as uint8_t,
    4 as i32 as uint8_t,
    4 as i32 as uint8_t,
    4 as i32 as uint8_t,
    4 as i32 as uint8_t,
    4 as i32 as uint8_t,
    4 as i32 as uint8_t,
    4 as i32 as uint8_t,
    4 as i32 as uint8_t,
    3 as i32 as uint8_t,
    3 as i32 as uint8_t,
    3 as i32 as uint8_t,
    3 as i32 as uint8_t,
    3 as i32 as uint8_t,
    3 as i32 as uint8_t,
    3 as i32 as uint8_t,
    3 as i32 as uint8_t,
    3 as i32 as uint8_t,
    3 as i32 as uint8_t,
    3 as i32 as uint8_t,
    3 as i32 as uint8_t,
    3 as i32 as uint8_t,
    3 as i32 as uint8_t,
    3 as i32 as uint8_t,
    3 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    1 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0 as i32 as uint8_t,
];
unsafe fn tree_merge_probs_impl(
    mut i: ::core::ffi::c_uint,
    mut tree: *const vpx_tree_index,
    mut pre_probs: *const vpx_prob,
    mut counts: *const ::core::ffi::c_uint,
    mut probs: *mut vpx_prob,
) -> ::core::ffi::c_uint {
    unsafe {
        let l: i32 = *tree.offset(i as isize) as i32;
        let left_count: ::core::ffi::c_uint = if l <= 0 as i32 {
            *counts.offset(-l as isize)
        } else {
            tree_merge_probs_impl(l as ::core::ffi::c_uint, tree, pre_probs, counts, probs)
                as ::core::ffi::c_uint
        };
        let r: i32 =
            *tree.offset(i.wrapping_add(1 as ::core::ffi::c_uint) as isize) as i32;
        let right_count: ::core::ffi::c_uint = if r <= 0 as i32 {
            *counts.offset(-r as isize)
        } else {
            tree_merge_probs_impl(r as ::core::ffi::c_uint, tree, pre_probs, counts, probs)
                as ::core::ffi::c_uint
        };
        let ct: [::core::ffi::c_uint; 2] = [left_count, right_count];
        *probs.offset((i >> 1 as i32) as isize) = mode_mv_merge_probs(
            *pre_probs.offset((i >> 1 as i32) as isize),
            &raw const ct as *const ::core::ffi::c_uint,
        );
        left_count.wrapping_add(right_count)
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vpx_tree_merge_probs(
    mut tree: *const vpx_tree_index,
    mut pre_probs: *const vpx_prob,
    mut counts: *const ::core::ffi::c_uint,
    mut probs: *mut vpx_prob,
) {
    unsafe {
        tree_merge_probs_impl(0 as ::core::ffi::c_uint, tree, pre_probs, counts, probs);
    }
}
