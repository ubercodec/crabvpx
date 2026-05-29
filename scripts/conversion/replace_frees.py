import re

def main():
    filepath = "src/vp8/decoder/threading.rs"
    with open(filepath, "r") as f: content = f.read()
    orig = content
    
    # Let's just fix the flat arrays first because those we CAN free properly.
    flat_arrays = [
        ("h_decoding_thread", "PthreadT"),
        ("h_event_start_decoding", "SemaphoreT"),
        ("mb_row_di", "MbRowDec"),
        ("de_thread_data", "DecodethreadData")
    ]
    
    for row_name, type_name in flat_arrays:
        pattern = re.compile(
            r"if !\(\*pbi\)\." + row_name + r"\.is_null\(\) \{\s*"
            r"vpx_free\(\(\*pbi\)\." + row_name + r" as \*mut c_void\);\s*"
            r"\(\*pbi\)\." + row_name + r" = ::core::ptr::null_mut::<[^>]+>\(\);\s*\}",
            re.MULTILINE
        )
        def replace_free(m):
            free_code = f"""if !(*pbi).{row_name}.is_null() {{
            let _ = Vec::from_raw_parts((*pbi).{row_name}, 0, (*pbi).decoding_thread_count as usize);
            (*pbi).{row_name} = ::core::ptr::null_mut();
        }}"""
            return free_code
        content = pattern.sub(replace_free, content)
        
    if orig != content:
        with open(filepath, "w") as f: f.write(content)
        print("Updated frees successfully")

if __name__ == "__main__": main()
