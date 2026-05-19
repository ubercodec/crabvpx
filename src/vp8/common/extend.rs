pub use crate::vp8::common::types::*;
pub type vpx_color_space = ::core::ffi::c_uint;
pub const VPX_CS_SRGB: vpx_color_space = 7;
pub const VPX_CS_RESERVED: vpx_color_space = 6;
pub const VPX_CS_BT_2020: vpx_color_space = 5;
pub const VPX_CS_SMPTE_240: vpx_color_space = 4;
pub const VPX_CS_SMPTE_170: vpx_color_space = 3;
pub const VPX_CS_BT_709: vpx_color_space = 2;
pub const VPX_CS_BT_601: vpx_color_space = 1;
pub const VPX_CS_UNKNOWN: vpx_color_space = 0;
pub type vpx_color_space_t = vpx_color_space;
pub type vpx_color_range = ::core::ffi::c_uint;
pub const VPX_CR_FULL_RANGE: vpx_color_range = 1;
pub const VPX_CR_STUDIO_RANGE: vpx_color_range = 0;
pub type vpx_color_range_t = vpx_color_range;
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;

fn copy_and_extend_plane_safe(
    src: &[u8],
    sp: usize,
    dst: &mut [u8],
    dp: usize,
    h: usize,
    w: usize,
    et: usize,
    el: usize,
    eb: usize,
    er: usize,
    interleave_step: usize,
) {
    // We will fill the active rows and their left/right borders.
    for r in 0..h {
        let src_row_start = r * sp;
        let dst_row_start = (et + r) * dp; // Row in dst_slice
        
        // 1. Left border
        let src_left_val = src[src_row_start];
        for i in 0..el {
            dst[dst_row_start + i] = src_left_val;
        }
        
        // 2. Active copy
        if interleave_step == 1 {
            let src_row = &src[src_row_start .. src_row_start + w];
            dst[dst_row_start + el .. dst_row_start + el + w].copy_from_slice(src_row);
        } else {
            for j in 0..w {
                dst[dst_row_start + el + j] = src[src_row_start + j * interleave_step];
            }
        }
        
        // 3. Right border
        let src_right_val = src[src_row_start + (w - 1) * interleave_step];
        for i in 0..er {
            dst[dst_row_start + el + w + i] = src_right_val;
        }
    }
    
    // Now we extend the top and bottom borders.
    let linesize = el + w + er;
    
    // 4. Top border (replicate the first active row of dst)
    let first_active_row_start = et * dp;
    for r in 0..et {
        let dst_row_start = r * dp;
        dst.copy_within(first_active_row_start .. first_active_row_start + linesize, dst_row_start);
    }
    
    // 5. Bottom border (replicate the last active row of dst)
    let last_active_row_start = (et + h - 1) * dp;
    for r in 0..eb {
        let dst_row_start = (et + h + r) * dp;
        dst.copy_within(last_active_row_start .. last_active_row_start + linesize, dst_row_start);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_copy_and_extend_frame(
    src: *mut YV12_BUFFER_CONFIG,
    dst: *mut YV12_BUFFER_CONFIG,
) {
    unsafe {
        let src = &*src;
        let dst = &mut *dst;
        
        let et = dst.border;
        let el = dst.border;
        let eb = dst.border + dst.y_height - src.y_height;
        let er = dst.border + dst.y_width - src.y_width;
        
        let chroma_step = if src.v_buffer.offset_from(src.u_buffer) == 1 {
            2
        } else {
            1
        };
        
        // Y plane
        {
            let src_len = (src.y_height - 1) as usize * src.y_stride as usize + src.y_width as usize;
            let src_slice = core::slice::from_raw_parts(src.y_buffer, src_len);
            
            let dst_origin = dst.y_buffer.offset(-((et * dst.y_stride + el) as isize));
            let total_h = et + src.y_height + eb;
            let dst_slice = core::slice::from_raw_parts_mut(dst_origin, total_h as usize * dst.y_stride as usize);
            
            copy_and_extend_plane_safe(
                src_slice,
                src.y_stride as usize,
                dst_slice,
                dst.y_stride as usize,
                src.y_height as usize,
                src.y_width as usize,
                et as usize,
                el as usize,
                eb as usize,
                er as usize,
                1,
            );
        }
        
        // U plane
        let et_uv = dst.border >> 1;
        let el_uv = dst.border >> 1;
        let eb_uv = (dst.border >> 1) + dst.uv_height - src.uv_height;
        let er_uv = (dst.border >> 1) + dst.uv_width - src.uv_width;
        
        {
            let src_len = (src.uv_height - 1) as usize * src.uv_stride as usize + (src.uv_width - 1) as usize * chroma_step as usize + 1;
            let src_slice = core::slice::from_raw_parts(src.u_buffer, src_len);
            
            let dst_origin = dst.u_buffer.offset(-((et_uv * dst.uv_stride + el_uv) as isize));
            let total_h = et_uv + src.uv_height + eb_uv;
            let dst_slice = core::slice::from_raw_parts_mut(dst_origin, total_h as usize * dst.uv_stride as usize);
            
            copy_and_extend_plane_safe(
                src_slice,
                src.uv_stride as usize,
                dst_slice,
                dst.uv_stride as usize,
                src.uv_height as usize,
                src.uv_width as usize,
                et_uv as usize,
                el_uv as usize,
                eb_uv as usize,
                er_uv as usize,
                chroma_step as usize,
            );
        }
        
        // V plane
        {
            let src_len = (src.uv_height - 1) as usize * src.uv_stride as usize + (src.uv_width - 1) as usize * chroma_step as usize + 1;
            let src_slice = core::slice::from_raw_parts(src.v_buffer, src_len);
            
            let dst_origin = dst.v_buffer.offset(-((et_uv * dst.uv_stride + el_uv) as isize));
            let total_h = et_uv + src.uv_height + eb_uv;
            let dst_slice = core::slice::from_raw_parts_mut(dst_origin, total_h as usize * dst.uv_stride as usize);
            
            copy_and_extend_plane_safe(
                src_slice,
                src.uv_stride as usize,
                dst_slice,
                dst.uv_stride as usize,
                src.uv_height as usize,
                src.uv_width as usize,
                et_uv as usize,
                el_uv as usize,
                eb_uv as usize,
                er_uv as usize,
                chroma_step as usize,
            );
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_copy_and_extend_frame_with_rect(
    src: *mut YV12_BUFFER_CONFIG,
    dst: *mut YV12_BUFFER_CONFIG,
    srcy: ::core::ffi::c_int,
    srcx: ::core::ffi::c_int,
    srch: ::core::ffi::c_int,
    srcw: ::core::ffi::c_int,
) {
    unsafe {
        let src = &*src;
        let dst = &mut *dst;
        
        let mut et = dst.border;
        let mut el = dst.border;
        let mut eb = dst.border + dst.y_height - src.y_height;
        let mut er = dst.border + dst.y_width - src.y_width;
        
        let chroma_step = if src.v_buffer.offset_from(src.u_buffer) == 1 {
            2
        } else {
            1
        };
        
        if srcy != 0 {
            et = 0;
        }
        if srcx != 0 {
            el = 0;
        }
        if srcy + srch != src.y_height {
            eb = 0;
        }
        if srcx + srcw != src.y_width {
            er = 0;
        }
        
        let src_y_offset = srcy * src.y_stride + srcx;
        let dst_y_offset = srcy * dst.y_stride + srcx;
        
        // Y plane
        {
            let src_start = src.y_buffer.offset(src_y_offset as isize);
            let src_len = (srch - 1) as usize * src.y_stride as usize + srcw as usize;
            let src_slice = core::slice::from_raw_parts(src_start, src_len);
            
            let dst_block_start = dst.y_buffer.offset(dst_y_offset as isize);
            let dst_origin = dst_block_start.offset(-((et * dst.y_stride + el) as isize));
            let total_h = et + srch + eb;
            let dst_slice = core::slice::from_raw_parts_mut(dst_origin, total_h as usize * dst.y_stride as usize);
            
            copy_and_extend_plane_safe(
                src_slice,
                src.y_stride as usize,
                dst_slice,
                dst.y_stride as usize,
                srch as usize,
                srcw as usize,
                et as usize,
                el as usize,
                eb as usize,
                er as usize,
                1,
            );
        }
        
        // UV dimensions
        let mut et_uv = et + 1 >> 1;
        let mut el_uv = el + 1 >> 1;
        let mut eb_uv = eb + 1 >> 1;
        let mut er_uv = er + 1 >> 1;
        let srch_uv = srch + 1 >> 1;
        let srcw_uv = srcw + 1 >> 1;
        
        let src_uv_offset = (srcy * src.uv_stride >> 1) + (srcx >> 1);
        let dst_uv_offset = (srcy * dst.uv_stride >> 1) + (srcx >> 1);
        
        // U plane
        {
            let src_start = src.u_buffer.offset(src_uv_offset as isize);
            let src_len = (srch_uv - 1) as usize * src.uv_stride as usize + (srcw_uv - 1) as usize * chroma_step as usize + 1;
            let src_slice = core::slice::from_raw_parts(src_start, src_len);
            
            let dst_block_start = dst.u_buffer.offset(dst_uv_offset as isize);
            let dst_origin = dst_block_start.offset(-((et_uv * dst.uv_stride + el_uv) as isize));
            let total_h = et_uv + srch_uv + eb_uv;
            let dst_slice = core::slice::from_raw_parts_mut(dst_origin, total_h as usize * dst.uv_stride as usize);
            
            copy_and_extend_plane_safe(
                src_slice,
                src.uv_stride as usize,
                dst_slice,
                dst.uv_stride as usize,
                srch_uv as usize,
                srcw_uv as usize,
                et_uv as usize,
                el_uv as usize,
                eb_uv as usize,
                er_uv as usize,
                chroma_step as usize,
            );
        }
        
        // V plane
        {
            let src_start = src.v_buffer.offset(src_uv_offset as isize);
            let src_len = (srch_uv - 1) as usize * src.uv_stride as usize + (srcw_uv - 1) as usize * chroma_step as usize + 1;
            let src_slice = core::slice::from_raw_parts(src_start, src_len);
            
            let dst_block_start = dst.v_buffer.offset(dst_uv_offset as isize);
            let dst_origin = dst_block_start.offset(-((et_uv * dst.uv_stride + el_uv) as isize));
            let total_h = et_uv + srch_uv + eb_uv;
            let dst_slice = core::slice::from_raw_parts_mut(dst_origin, total_h as usize * dst.uv_stride as usize);
            
            copy_and_extend_plane_safe(
                src_slice,
                src.uv_stride as usize,
                dst_slice,
                dst.uv_stride as usize,
                srch_uv as usize,
                srcw_uv as usize,
                et_uv as usize,
                el_uv as usize,
                eb_uv as usize,
                er_uv as usize,
                chroma_step as usize,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub fn vp8_extend_mb_row(
    ybf: &mut YV12_BUFFER_CONFIG,
    mb_row: i32,
) {
    let y_stride = ybf.y_stride as usize;
    let uv_stride = ybf.uv_stride as usize;
    let y_width = ybf.y_width as usize;
    let uv_width = ybf.uv_width as usize;
    let border = ybf.border as usize;
    let mb_row = mb_row as usize;

    // Y plane border extension
    {
        let y_slice = ybf.y_slice_mut_safe();
        
        // Y plane row 14
        {
            let row_idx = border + mb_row * 16 + 14;
            let row_start = row_idx * y_stride;
            let src_val = y_slice[row_start + border + y_width - 1];
            let dst_start = row_start + border + y_width;
            for i in 0..4 {
                y_slice[dst_start + i] = src_val;
            }
        }
        // Y plane row 15
        {
            let row_idx = border + mb_row * 16 + 15;
            let row_start = row_idx * y_stride;
            let src_val = y_slice[row_start + border + y_width - 1];
            let dst_start = row_start + border + y_width;
            for i in 0..4 {
                y_slice[dst_start + i] = src_val;
            }
        }
    }

    let uv_border = border / 2;

    // U plane border extension
    {
        let u_slice = ybf.u_slice_mut_safe();
        
        // U plane row 6
        {
            let row_idx = uv_border + mb_row * 8 + 6;
            let row_start = row_idx * uv_stride;
            let src_val = u_slice[row_start + uv_border + uv_width - 1];
            let dst_start = row_start + uv_border + uv_width;
            for i in 0..4 {
                u_slice[dst_start + i] = src_val;
            }
        }
        // U plane row 7
        {
            let row_idx = uv_border + mb_row * 8 + 7;
            let row_start = row_idx * uv_stride;
            let src_val = u_slice[row_start + uv_border + uv_width - 1];
            let dst_start = row_start + uv_border + uv_width;
            for i in 0..4 {
                u_slice[dst_start + i] = src_val;
            }
        }
    }

    // V plane border extension
    {
        let v_slice = ybf.v_slice_mut_safe();
        
        // V plane row 6
        {
            let row_idx = uv_border + mb_row * 8 + 6;
            let row_start = row_idx * uv_stride;
            let src_val = v_slice[row_start + uv_border + uv_width - 1];
            let dst_start = row_start + uv_border + uv_width;
            for i in 0..4 {
                v_slice[dst_start + i] = src_val;
            }
        }
        // V plane row 7
        {
            let row_idx = uv_border + mb_row * 8 + 7;
            let row_start = row_idx * uv_stride;
            let src_val = v_slice[row_start + uv_border + uv_width - 1];
            let dst_start = row_start + uv_border + uv_width;
            for i in 0..4 {
                v_slice[dst_start + i] = src_val;
            }
        }
    }
}
