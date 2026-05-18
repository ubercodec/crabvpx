pub const VP8_FILTER_WEIGHT: i32 = 128 as i32;
pub const VP8_FILTER_SHIFT: i32 = 7 as i32;
#[unsafe(no_mangle)]
pub static mut vp8_bilinear_filters: [[::core::ffi::c_short; 2]; 8] = [
    [
        128 as i32 as ::core::ffi::c_short,
        0 as i32 as ::core::ffi::c_short,
    ],
    [
        112 as i32 as ::core::ffi::c_short,
        16 as i32 as ::core::ffi::c_short,
    ],
    [
        96 as i32 as ::core::ffi::c_short,
        32 as i32 as ::core::ffi::c_short,
    ],
    [
        80 as i32 as ::core::ffi::c_short,
        48 as i32 as ::core::ffi::c_short,
    ],
    [
        64 as i32 as ::core::ffi::c_short,
        64 as i32 as ::core::ffi::c_short,
    ],
    [
        48 as i32 as ::core::ffi::c_short,
        80 as i32 as ::core::ffi::c_short,
    ],
    [
        32 as i32 as ::core::ffi::c_short,
        96 as i32 as ::core::ffi::c_short,
    ],
    [
        16 as i32 as ::core::ffi::c_short,
        112 as i32 as ::core::ffi::c_short,
    ],
];
#[unsafe(no_mangle)]
pub static mut vp8_sub_pel_filters: [[::core::ffi::c_short; 6]; 8] = [
    [
        0 as i32 as ::core::ffi::c_short,
        0 as i32 as ::core::ffi::c_short,
        128 as i32 as ::core::ffi::c_short,
        0 as i32 as ::core::ffi::c_short,
        0 as i32 as ::core::ffi::c_short,
        0 as i32 as ::core::ffi::c_short,
    ],
    [
        0 as i32 as ::core::ffi::c_short,
        -(6 as i32) as ::core::ffi::c_short,
        123 as i32 as ::core::ffi::c_short,
        12 as i32 as ::core::ffi::c_short,
        -(1 as i32) as ::core::ffi::c_short,
        0 as i32 as ::core::ffi::c_short,
    ],
    [
        2 as i32 as ::core::ffi::c_short,
        -(11 as i32) as ::core::ffi::c_short,
        108 as i32 as ::core::ffi::c_short,
        36 as i32 as ::core::ffi::c_short,
        -(8 as i32) as ::core::ffi::c_short,
        1 as i32 as ::core::ffi::c_short,
    ],
    [
        0 as i32 as ::core::ffi::c_short,
        -(9 as i32) as ::core::ffi::c_short,
        93 as i32 as ::core::ffi::c_short,
        50 as i32 as ::core::ffi::c_short,
        -(6 as i32) as ::core::ffi::c_short,
        0 as i32 as ::core::ffi::c_short,
    ],
    [
        3 as i32 as ::core::ffi::c_short,
        -(16 as i32) as ::core::ffi::c_short,
        77 as i32 as ::core::ffi::c_short,
        77 as i32 as ::core::ffi::c_short,
        -(16 as i32) as ::core::ffi::c_short,
        3 as i32 as ::core::ffi::c_short,
    ],
    [
        0 as i32 as ::core::ffi::c_short,
        -(6 as i32) as ::core::ffi::c_short,
        50 as i32 as ::core::ffi::c_short,
        93 as i32 as ::core::ffi::c_short,
        -(9 as i32) as ::core::ffi::c_short,
        0 as i32 as ::core::ffi::c_short,
    ],
    [
        1 as i32 as ::core::ffi::c_short,
        -(8 as i32) as ::core::ffi::c_short,
        36 as i32 as ::core::ffi::c_short,
        108 as i32 as ::core::ffi::c_short,
        -(11 as i32) as ::core::ffi::c_short,
        2 as i32 as ::core::ffi::c_short,
    ],
    [
        0 as i32 as ::core::ffi::c_short,
        -(1 as i32) as ::core::ffi::c_short,
        12 as i32 as ::core::ffi::c_short,
        123 as i32 as ::core::ffi::c_short,
        -(6 as i32) as ::core::ffi::c_short,
        0 as i32 as ::core::ffi::c_short,
    ],
];
unsafe fn filter_block2d_first_pass(
    mut src_ptr: *mut ::core::ffi::c_uchar,
    mut output_ptr: *mut i32,
    mut src_pixels_per_line: ::core::ffi::c_uint,
    mut pixel_step: ::core::ffi::c_uint,
    mut output_height: ::core::ffi::c_uint,
    mut output_width: ::core::ffi::c_uint,
    mut vp8_filter: *const ::core::ffi::c_short,
) {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        let mut j: ::core::ffi::c_uint = 0;
        let mut Temp: i32 = 0;
        i = 0 as ::core::ffi::c_uint;
        while i < output_height {
            j = 0 as ::core::ffi::c_uint;
            while j < output_width {
                Temp = *src_ptr.offset(
                    (-(2 as i32) * pixel_step as i32) as isize,
                ) as i32
                    * *vp8_filter.offset(0 as i32 as isize) as i32
                    + *src_ptr.offset(
                        (-(1 as i32) * pixel_step as i32) as isize,
                    ) as i32
                        * *vp8_filter.offset(1 as i32 as isize)
                            as i32
                    + *src_ptr.offset(0 as i32 as isize) as i32
                        * *vp8_filter.offset(2 as i32 as isize)
                            as i32
                    + *src_ptr.offset(pixel_step as isize) as i32
                        * *vp8_filter.offset(3 as i32 as isize)
                            as i32
                    + *src_ptr.offset((2 as ::core::ffi::c_uint).wrapping_mul(pixel_step) as isize)
                        as i32
                        * *vp8_filter.offset(4 as i32 as isize)
                            as i32
                    + *src_ptr.offset((3 as ::core::ffi::c_uint).wrapping_mul(pixel_step) as isize)
                        as i32
                        * *vp8_filter.offset(5 as i32 as isize)
                            as i32
                    + (VP8_FILTER_WEIGHT >> 1 as i32);
                Temp >>= VP8_FILTER_SHIFT;
                if Temp < 0 as i32 {
                    Temp = 0 as i32;
                } else if Temp > 255 as i32 {
                    Temp = 255 as i32;
                }
                *output_ptr.offset(j as isize) = Temp;
                src_ptr = src_ptr.offset(1);
                j = j.wrapping_add(1);
            }
            src_ptr = src_ptr.offset(src_pixels_per_line.wrapping_sub(output_width) as isize);
            output_ptr = output_ptr.offset(output_width as isize);
            i = i.wrapping_add(1);
        }
    }
}
unsafe fn filter_block2d_second_pass(
    mut src_ptr: *mut i32,
    mut output_ptr: *mut ::core::ffi::c_uchar,
    mut output_pitch: i32,
    mut src_pixels_per_line: ::core::ffi::c_uint,
    mut pixel_step: ::core::ffi::c_uint,
    mut output_height: ::core::ffi::c_uint,
    mut output_width: ::core::ffi::c_uint,
    mut vp8_filter: *const ::core::ffi::c_short,
) {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        let mut j: ::core::ffi::c_uint = 0;
        let mut Temp: i32 = 0;
        i = 0 as ::core::ffi::c_uint;
        while i < output_height {
            j = 0 as ::core::ffi::c_uint;
            while j < output_width {
                Temp = *src_ptr.offset(
                    (-(2 as i32) * pixel_step as i32) as isize,
                ) * *vp8_filter.offset(0 as i32 as isize)
                    as i32
                    + *src_ptr.offset(
                        (-(1 as i32) * pixel_step as i32) as isize,
                    ) * *vp8_filter.offset(1 as i32 as isize)
                        as i32
                    + *src_ptr.offset(0 as i32 as isize)
                        * *vp8_filter.offset(2 as i32 as isize)
                            as i32
                    + *src_ptr.offset(pixel_step as isize)
                        * *vp8_filter.offset(3 as i32 as isize)
                            as i32
                    + *src_ptr.offset((2 as ::core::ffi::c_uint).wrapping_mul(pixel_step) as isize)
                        * *vp8_filter.offset(4 as i32 as isize)
                            as i32
                    + *src_ptr.offset((3 as ::core::ffi::c_uint).wrapping_mul(pixel_step) as isize)
                        * *vp8_filter.offset(5 as i32 as isize)
                            as i32
                    + (VP8_FILTER_WEIGHT >> 1 as i32);
                Temp >>= VP8_FILTER_SHIFT;
                if Temp < 0 as i32 {
                    Temp = 0 as i32;
                } else if Temp > 255 as i32 {
                    Temp = 255 as i32;
                }
                *output_ptr.offset(j as isize) = Temp as ::core::ffi::c_uchar;
                src_ptr = src_ptr.offset(1);
                j = j.wrapping_add(1);
            }
            src_ptr = src_ptr.offset(src_pixels_per_line.wrapping_sub(output_width) as isize);
            output_ptr = output_ptr.offset(output_pitch as isize);
            i = i.wrapping_add(1);
        }
    }
}
unsafe fn filter_block2d(
    mut src_ptr: *mut ::core::ffi::c_uchar,
    mut output_ptr: *mut ::core::ffi::c_uchar,
    mut src_pixels_per_line: ::core::ffi::c_uint,
    mut output_pitch: i32,
    mut HFilter: *const ::core::ffi::c_short,
    mut VFilter: *const ::core::ffi::c_short,
) {
    unsafe {
        let mut FData: [i32; 36] = [0; 36];
        filter_block2d_first_pass(
            src_ptr
                .offset(-((2 as ::core::ffi::c_uint).wrapping_mul(src_pixels_per_line) as isize)),
            &raw mut FData as *mut i32,
            src_pixels_per_line,
            1 as ::core::ffi::c_uint,
            9 as ::core::ffi::c_uint,
            4 as ::core::ffi::c_uint,
            HFilter,
        );
        filter_block2d_second_pass(
            (&raw mut FData as *mut i32).offset(8 as i32 as isize),
            output_ptr,
            output_pitch,
            4 as ::core::ffi::c_uint,
            4 as ::core::ffi::c_uint,
            4 as ::core::ffi::c_uint,
            4 as ::core::ffi::c_uint,
            VFilter,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_sixtap_predict4x4_c(
    mut src_ptr: *mut ::core::ffi::c_uchar,
    mut src_pixels_per_line: i32,
    mut xoffset: i32,
    mut yoffset: i32,
    mut dst_ptr: *mut ::core::ffi::c_uchar,
    mut dst_pitch: i32,
) {
    unsafe {
        let mut HFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        let mut VFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        HFilter = &raw const *(&raw const vp8_sub_pel_filters as *const [::core::ffi::c_short; 6])
            .offset(xoffset as isize) as *const ::core::ffi::c_short;
        VFilter = &raw const *(&raw const vp8_sub_pel_filters as *const [::core::ffi::c_short; 6])
            .offset(yoffset as isize) as *const ::core::ffi::c_short;
        filter_block2d(
            src_ptr,
            dst_ptr,
            src_pixels_per_line as ::core::ffi::c_uint,
            dst_pitch,
            HFilter,
            VFilter,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_sixtap_predict8x8_c(
    mut src_ptr: *mut ::core::ffi::c_uchar,
    mut src_pixels_per_line: i32,
    mut xoffset: i32,
    mut yoffset: i32,
    mut dst_ptr: *mut ::core::ffi::c_uchar,
    mut dst_pitch: i32,
) {
    unsafe {
        let mut HFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        let mut VFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        let mut FData: [i32; 208] = [0; 208];
        HFilter = &raw const *(&raw const vp8_sub_pel_filters as *const [::core::ffi::c_short; 6])
            .offset(xoffset as isize) as *const ::core::ffi::c_short;
        VFilter = &raw const *(&raw const vp8_sub_pel_filters as *const [::core::ffi::c_short; 6])
            .offset(yoffset as isize) as *const ::core::ffi::c_short;
        filter_block2d_first_pass(
            src_ptr.offset(-((2 as i32 * src_pixels_per_line) as isize)),
            &raw mut FData as *mut i32,
            src_pixels_per_line as ::core::ffi::c_uint,
            1 as ::core::ffi::c_uint,
            13 as ::core::ffi::c_uint,
            8 as ::core::ffi::c_uint,
            HFilter,
        );
        filter_block2d_second_pass(
            (&raw mut FData as *mut i32).offset(16 as i32 as isize),
            dst_ptr,
            dst_pitch,
            8 as ::core::ffi::c_uint,
            8 as ::core::ffi::c_uint,
            8 as ::core::ffi::c_uint,
            8 as ::core::ffi::c_uint,
            VFilter,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_sixtap_predict8x4_c(
    mut src_ptr: *mut ::core::ffi::c_uchar,
    mut src_pixels_per_line: i32,
    mut xoffset: i32,
    mut yoffset: i32,
    mut dst_ptr: *mut ::core::ffi::c_uchar,
    mut dst_pitch: i32,
) {
    unsafe {
        let mut HFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        let mut VFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        let mut FData: [i32; 208] = [0; 208];
        HFilter = &raw const *(&raw const vp8_sub_pel_filters as *const [::core::ffi::c_short; 6])
            .offset(xoffset as isize) as *const ::core::ffi::c_short;
        VFilter = &raw const *(&raw const vp8_sub_pel_filters as *const [::core::ffi::c_short; 6])
            .offset(yoffset as isize) as *const ::core::ffi::c_short;
        filter_block2d_first_pass(
            src_ptr.offset(-((2 as i32 * src_pixels_per_line) as isize)),
            &raw mut FData as *mut i32,
            src_pixels_per_line as ::core::ffi::c_uint,
            1 as ::core::ffi::c_uint,
            9 as ::core::ffi::c_uint,
            8 as ::core::ffi::c_uint,
            HFilter,
        );
        filter_block2d_second_pass(
            (&raw mut FData as *mut i32).offset(16 as i32 as isize),
            dst_ptr,
            dst_pitch,
            8 as ::core::ffi::c_uint,
            8 as ::core::ffi::c_uint,
            4 as ::core::ffi::c_uint,
            8 as ::core::ffi::c_uint,
            VFilter,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_sixtap_predict16x16_c(
    mut src_ptr: *mut ::core::ffi::c_uchar,
    mut src_pixels_per_line: i32,
    mut xoffset: i32,
    mut yoffset: i32,
    mut dst_ptr: *mut ::core::ffi::c_uchar,
    mut dst_pitch: i32,
) {
    unsafe {
        let mut HFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        let mut VFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        let mut FData: [i32; 504] = [0; 504];
        HFilter = &raw const *(&raw const vp8_sub_pel_filters as *const [::core::ffi::c_short; 6])
            .offset(xoffset as isize) as *const ::core::ffi::c_short;
        VFilter = &raw const *(&raw const vp8_sub_pel_filters as *const [::core::ffi::c_short; 6])
            .offset(yoffset as isize) as *const ::core::ffi::c_short;
        filter_block2d_first_pass(
            src_ptr.offset(-((2 as i32 * src_pixels_per_line) as isize)),
            &raw mut FData as *mut i32,
            src_pixels_per_line as ::core::ffi::c_uint,
            1 as ::core::ffi::c_uint,
            21 as ::core::ffi::c_uint,
            16 as ::core::ffi::c_uint,
            HFilter,
        );
        filter_block2d_second_pass(
            (&raw mut FData as *mut i32).offset(32 as i32 as isize),
            dst_ptr,
            dst_pitch,
            16 as ::core::ffi::c_uint,
            16 as ::core::ffi::c_uint,
            16 as ::core::ffi::c_uint,
            16 as ::core::ffi::c_uint,
            VFilter,
        );
    }
}
unsafe fn filter_block2d_bil_first_pass(
    mut src_ptr: *mut ::core::ffi::c_uchar,
    mut dst_ptr: *mut ::core::ffi::c_ushort,
    mut src_stride: ::core::ffi::c_uint,
    mut height: ::core::ffi::c_uint,
    mut width: ::core::ffi::c_uint,
    mut vp8_filter: *const ::core::ffi::c_short,
) {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        let mut j: ::core::ffi::c_uint = 0;
        i = 0 as ::core::ffi::c_uint;
        while i < height {
            j = 0 as ::core::ffi::c_uint;
            while j < width {
                *dst_ptr.offset(j as isize) = ((*src_ptr.offset(0 as i32 as isize)
                    as i32
                    * *vp8_filter.offset(0 as i32 as isize) as i32
                    + *src_ptr.offset(1 as i32 as isize) as i32
                        * *vp8_filter.offset(1 as i32 as isize)
                            as i32
                    + VP8_FILTER_WEIGHT / 2 as i32)
                    >> VP8_FILTER_SHIFT)
                    as ::core::ffi::c_ushort;
                src_ptr = src_ptr.offset(1);
                j = j.wrapping_add(1);
            }
            src_ptr = src_ptr.offset(src_stride.wrapping_sub(width) as isize);
            dst_ptr = dst_ptr.offset(width as isize);
            i = i.wrapping_add(1);
        }
    }
}
unsafe fn filter_block2d_bil_second_pass(
    mut src_ptr: *mut ::core::ffi::c_ushort,
    mut dst_ptr: *mut ::core::ffi::c_uchar,
    mut dst_pitch: i32,
    mut height: ::core::ffi::c_uint,
    mut width: ::core::ffi::c_uint,
    mut vp8_filter: *const ::core::ffi::c_short,
) {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        let mut j: ::core::ffi::c_uint = 0;
        let mut Temp: i32 = 0;
        i = 0 as ::core::ffi::c_uint;
        while i < height {
            j = 0 as ::core::ffi::c_uint;
            while j < width {
                Temp = *src_ptr.offset(0 as i32 as isize) as i32
                    * *vp8_filter.offset(0 as i32 as isize) as i32
                    + *src_ptr.offset(width as isize) as i32
                        * *vp8_filter.offset(1 as i32 as isize)
                            as i32
                    + VP8_FILTER_WEIGHT / 2 as i32;
                *dst_ptr.offset(j as isize) =
                    (Temp >> VP8_FILTER_SHIFT) as ::core::ffi::c_uint as ::core::ffi::c_uchar;
                src_ptr = src_ptr.offset(1);
                j = j.wrapping_add(1);
            }
            dst_ptr = dst_ptr.offset(dst_pitch as isize);
            i = i.wrapping_add(1);
        }
    }
}
unsafe fn filter_block2d_bil(
    mut src_ptr: *mut ::core::ffi::c_uchar,
    mut dst_ptr: *mut ::core::ffi::c_uchar,
    mut src_pitch: ::core::ffi::c_uint,
    mut dst_pitch: ::core::ffi::c_uint,
    mut HFilter: *const ::core::ffi::c_short,
    mut VFilter: *const ::core::ffi::c_short,
    mut Width: i32,
    mut Height: i32,
) {
    unsafe {
        let mut FData: [::core::ffi::c_ushort; 272] = [0; 272];
        filter_block2d_bil_first_pass(
            src_ptr,
            &raw mut FData as *mut ::core::ffi::c_ushort,
            src_pitch,
            (Height + 1 as i32) as ::core::ffi::c_uint,
            Width as ::core::ffi::c_uint,
            HFilter,
        );
        filter_block2d_bil_second_pass(
            &raw mut FData as *mut ::core::ffi::c_ushort,
            dst_ptr,
            dst_pitch as i32,
            Height as ::core::ffi::c_uint,
            Width as ::core::ffi::c_uint,
            VFilter,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_bilinear_predict4x4_c(
    mut src_ptr: *mut ::core::ffi::c_uchar,
    mut src_pixels_per_line: i32,
    mut xoffset: i32,
    mut yoffset: i32,
    mut dst_ptr: *mut ::core::ffi::c_uchar,
    mut dst_pitch: i32,
) {
    unsafe {
        let mut HFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        let mut VFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        HFilter = &raw const *(&raw const vp8_bilinear_filters as *const [::core::ffi::c_short; 2])
            .offset(xoffset as isize) as *const ::core::ffi::c_short;
        VFilter = &raw const *(&raw const vp8_bilinear_filters as *const [::core::ffi::c_short; 2])
            .offset(yoffset as isize) as *const ::core::ffi::c_short;
        filter_block2d_bil(
            src_ptr,
            dst_ptr,
            src_pixels_per_line as ::core::ffi::c_uint,
            dst_pitch as ::core::ffi::c_uint,
            HFilter,
            VFilter,
            4 as i32,
            4 as i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_bilinear_predict8x8_c(
    mut src_ptr: *mut ::core::ffi::c_uchar,
    mut src_pixels_per_line: i32,
    mut xoffset: i32,
    mut yoffset: i32,
    mut dst_ptr: *mut ::core::ffi::c_uchar,
    mut dst_pitch: i32,
) {
    unsafe {
        let mut HFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        let mut VFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        HFilter = &raw const *(&raw const vp8_bilinear_filters as *const [::core::ffi::c_short; 2])
            .offset(xoffset as isize) as *const ::core::ffi::c_short;
        VFilter = &raw const *(&raw const vp8_bilinear_filters as *const [::core::ffi::c_short; 2])
            .offset(yoffset as isize) as *const ::core::ffi::c_short;
        filter_block2d_bil(
            src_ptr,
            dst_ptr,
            src_pixels_per_line as ::core::ffi::c_uint,
            dst_pitch as ::core::ffi::c_uint,
            HFilter,
            VFilter,
            8 as i32,
            8 as i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_bilinear_predict8x4_c(
    mut src_ptr: *mut ::core::ffi::c_uchar,
    mut src_pixels_per_line: i32,
    mut xoffset: i32,
    mut yoffset: i32,
    mut dst_ptr: *mut ::core::ffi::c_uchar,
    mut dst_pitch: i32,
) {
    unsafe {
        let mut HFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        let mut VFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        HFilter = &raw const *(&raw const vp8_bilinear_filters as *const [::core::ffi::c_short; 2])
            .offset(xoffset as isize) as *const ::core::ffi::c_short;
        VFilter = &raw const *(&raw const vp8_bilinear_filters as *const [::core::ffi::c_short; 2])
            .offset(yoffset as isize) as *const ::core::ffi::c_short;
        filter_block2d_bil(
            src_ptr,
            dst_ptr,
            src_pixels_per_line as ::core::ffi::c_uint,
            dst_pitch as ::core::ffi::c_uint,
            HFilter,
            VFilter,
            8 as i32,
            4 as i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn vp8_bilinear_predict16x16_c(
    mut src_ptr: *mut ::core::ffi::c_uchar,
    mut src_pixels_per_line: i32,
    mut xoffset: i32,
    mut yoffset: i32,
    mut dst_ptr: *mut ::core::ffi::c_uchar,
    mut dst_pitch: i32,
) {
    unsafe {
        let mut HFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        let mut VFilter: *const ::core::ffi::c_short = ::core::ptr::null::<::core::ffi::c_short>();
        HFilter = &raw const *(&raw const vp8_bilinear_filters as *const [::core::ffi::c_short; 2])
            .offset(xoffset as isize) as *const ::core::ffi::c_short;
        VFilter = &raw const *(&raw const vp8_bilinear_filters as *const [::core::ffi::c_short; 2])
            .offset(yoffset as isize) as *const ::core::ffi::c_short;
        filter_block2d_bil(
            src_ptr,
            dst_ptr,
            src_pixels_per_line as ::core::ffi::c_uint,
            dst_pitch as ::core::ffi::c_uint,
            HFilter,
            VFilter,
            16 as i32,
            16 as i32,
        );
    }
}
