import re

def main():
    filepath = "src/vp8/decoder/threading.rs"
    with open(filepath, "r") as f:
        content = f.read()
    orig = content
    
    # Define replacements for allocation and deallocation of multi-dimensional arrays
    
    # Replacements for `mt_uabove_row`, `mt_vabove_row` allocations (u and v rows)
    # The target structure is:
    #             (*pbi).mt_uabove_row = vpx_calloc(...)
    #             if (*pbi).mt_uabove_row.is_null() { ... }
    #             i = 0 ...
    #             while i < (*pc).mb_rows {
    #                 let freshX = &mut *(*pbi).mt_uabove_row.offset(i as isize);
    #                 *freshX = vpx_memalign(...)
    #                 if (*(*pbi).mt_uabove_row.offset(i as isize)).is_null() { ... }
    #                 core::ptr::write_bytes(...)
    #                 i += 1;
    #             }
    
    for row_name in ["mt_uabove_row", "mt_vabove_row"]:
        pattern = re.compile(
            r"\(\*pbi\)\." + row_name + r" = vpx_calloc\([^)]+\) as \*mut \*mut u8;\s*"
            r"if \(\*pbi\)\." + row_name + r"\.is_null\(\) \{[^\}]+\}\s*"
            r"i = 0 as i32;\s*"
            r"while i < \(\*pc\)\.mb_rows \{[^\}]+vpx_memalign\([^)]+\)[^\}]+if[^\}]+is_null\(\)[^\}]+\}[^\}]+write_bytes\([^)]+\);\s*i \+= 1;\s*\}",
            re.MULTILINE
        )
        def replace_alloc(m):
            alloc_code = f"""            let row_count = (*pc).mb_rows as usize;
            
            let mut vec = Vec::<*mut u8>::with_capacity(row_count);
            vec.resize(row_count, core::ptr::null_mut());
            (*pbi).{row_name} = vec.as_mut_ptr();
            core::mem::forget(vec);
            
            if (*pbi).{row_name}.is_null() {{
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate (pbi->{row_name})\\0" as *const u8 as *const i8,
                );
            }}
            i = 0 as i32;
            let uv_alloc_size = ((uv_width + 32 as i32) as usize).wrapping_mul(::core::mem::size_of::<u8>() as usize);
            while i < (*pc).mb_rows {{
                let mut row_vec = Vec::<u8>::with_capacity(uv_alloc_size + 15);
                let base_ptr = row_vec.as_mut_ptr();
                core::mem::forget(row_vec);
                let aligned_ptr = ((base_ptr as usize + 15) & !15) as *mut u8;
                
                core::ptr::write_bytes(aligned_ptr, 0 as u8, uv_alloc_size);
                *(*pbi).{row_name}.offset(i as isize) = aligned_ptr;
                
                i += 1;
            }}"""
            return alloc_code
        content = pattern.sub(replace_alloc, content)

    # Replacements for `mt_yleft_col`, `mt_uleft_col`, `mt_vleft_col` allocations
    for col_name in ["mt_yleft_col", "mt_uleft_col", "mt_vleft_col"]:
        size_multiplier = "16" if col_name == "mt_yleft_col" else "8"
        pattern = re.compile(
            r"\(\*pbi\)\." + col_name + r" = vpx_calloc\([^)]+\) as \*mut \*mut u8;\s*"
            r"if \(\*pbi\)\." + col_name + r"\.is_null\(\) \{[^\}]+\}\s*"
            r"i = 0 as i32;\s*"
            r"while i < \(\*pc\)\.mb_rows \{[^\}]+vpx_calloc\([^)]+\)[^\}]+if[^\}]+is_null\(\)[^\}]+\}[^\}]+\}",
            re.MULTILINE
        )
        def replace_col_alloc(m):
            alloc_code = f"""            let row_count = (*pc).mb_rows as usize;
            
            let mut vec = Vec::<*mut u8>::with_capacity(row_count);
            vec.resize(row_count, core::ptr::null_mut());
            (*pbi).{col_name} = vec.as_mut_ptr();
            core::mem::forget(vec);
            
            if (*pbi).{col_name}.is_null() {{
                vpx_internal_error(
                    &raw mut (*pbi).common.error,
                    VPX_CODEC_MEM_ERROR,
                    b"Failed to allocate (pbi->{col_name})\\0" as *const u8 as *const i8,
                );
            }}
            i = 0 as i32;
            let col_alloc_size = (::core::mem::size_of::<u8>() as usize).wrapping_mul({size_multiplier} as usize);
            while i < (*pc).mb_rows {{
                let mut col_vec = Vec::<u8>::with_capacity(col_alloc_size);
                col_vec.resize(col_alloc_size, 0);
                *(*pbi).{col_name}.offset(i as isize) = col_vec.as_mut_ptr();
                core::mem::forget(col_vec);
                i += 1;
            }}"""
            return alloc_code
        content = pattern.sub(replace_col_alloc, content)

    if orig != content:
        with open(filepath, "w") as f:
            f.write(content)
        print("Updated allocations successfully")

if __name__ == "__main__":
    main()