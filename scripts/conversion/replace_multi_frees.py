import re

def main():
    filepath = "src/vp8/decoder/threading.rs"
    with open(filepath, "r") as f: content = f.read()
    orig = content
    
    multi_arrays = [
        ("mt_yabove_row", "let y_alloc_size = ((width + ((32 as i32) << 1 as i32)) as usize).wrapping_mul(::core::mem::size_of::<u8>() as usize);", "y_alloc_size + 15"),
        ("mt_uabove_row", "let uv_alloc_size = ((uv_width + 32 as i32) as usize).wrapping_mul(::core::mem::size_of::<u8>() as usize);", "uv_alloc_size + 15"),
        ("mt_vabove_row", "let uv_alloc_size = ((uv_width + 32 as i32) as usize).wrapping_mul(::core::mem::size_of::<u8>() as usize);", "uv_alloc_size + 15"),
        ("mt_yleft_col", "let col_alloc_size = (::core::mem::size_of::<u8>() as usize).wrapping_mul(16 as usize);", "col_alloc_size"),
        ("mt_uleft_col", "let col_alloc_size = (::core::mem::size_of::<u8>() as usize).wrapping_mul(8 as usize);", "col_alloc_size"),
        ("mt_vleft_col", "let col_alloc_size = (::core::mem::size_of::<u8>() as usize).wrapping_mul(8 as usize);", "col_alloc_size")
    ]
    
    for row_name, alloc_calc, capacity in multi_arrays:
        pattern = re.compile(
            r"if !\(\*pbi\)\." + row_name + r"\.is_null\(\) \{\s*"
            r"i = 0 as i32;\s*"
            r"while i < mb_rows \{\s*"
            r"vpx_free\(\*\(\*pbi\)\." + row_name + r"\.offset\(i as isize\) as \*mut c_void\);\s*"
            r"let fresh[0-9]+ = &mut \*\(\*pbi\)\." + row_name + r"\.offset\(i as isize\);\s*"
            r"\*fresh[0-9]+ = ::core::ptr::null_mut::<u8>\(\);\s*"
            r"i \+= 1;\s*"
            r"\}\s*"
            r"vpx_free\(\(\*pbi\)\." + row_name + r" as \*mut c_void\);\s*"
            r"\(\*pbi\)\." + row_name + r" = ::core::ptr::null_mut::<\*mut u8>\(\);\s*\}",
            re.MULTILINE
        )
        def replace_multi(m):
            free_code = f"""if !(*pbi).{row_name}.is_null() {{
            i = 0 as i32;
            while i < mb_rows {{
                let ptr = *(*pbi).{row_name}.offset(i as isize);
                if !ptr.is_null() {{
                    // NOTE: we leak the inner arrays here temporarily because we don't have the base pointer saved.
                    // Vec::from_raw_parts would crash on the aligned pointer!
                }}
                let fresh = &mut *(*pbi).{row_name}.offset(i as isize);
                *fresh = ::core::ptr::null_mut::<u8>();
                i += 1;
            }}
            let _ = Vec::from_raw_parts((*pbi).{row_name}, 0, mb_rows as usize);
            (*pbi).{row_name} = ::core::ptr::null_mut::<*mut u8>();
        }}"""
            return free_code
        content = pattern.sub(replace_multi, content)

    # replace mt_current_mb_col
    pattern_col = re.compile(
        r"vpx_free\(\(\*pbi\)\.mt_current_mb_col as \*mut c_void\);\s*"
        r"\(\*pbi\)\.mt_current_mb_col = ::core::ptr::null_mut::<VpxAtomicInt>\(\);"
    )
    def replace_col(m):
        return f"""if !(*pbi).mt_current_mb_col.is_null() {{
            let _ = Vec::from_raw_parts((*pbi).mt_current_mb_col, 0, mb_rows as usize);
            (*pbi).mt_current_mb_col = ::core::ptr::null_mut::<VpxAtomicInt>();
        }}"""
    content = pattern_col.sub(replace_col, content)
    
    if orig != content:
        with open(filepath, "w") as f: f.write(content)
        print("Updated multi frees successfully")

if __name__ == "__main__": main()
