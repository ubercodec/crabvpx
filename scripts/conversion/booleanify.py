import os, glob, re

def booleanify(struct_field_map):
    # struct_field_map: {struct_name: [field1, field2, ...]}
    
    all_fields = []
    for fields in struct_field_map.values():
        all_fields.extend(fields)
    all_fields = sorted(list(set(all_fields)), key=len, reverse=True)

    count_files = 0
    for filepath in glob.glob("src/**/*.rs", recursive=True):
        if "scripts" in filepath: continue
        with open(filepath, "r") as f:
            content = f.read()
        
        orig = content
        
        # 1. Update struct definitions
        for struct_name, fields in struct_field_map.items():
            pattern = re.compile(r"(pub\s+struct\s+" + struct_name + r"\s*\{[\s\S]*?\})")
            def repl_struct(m):
                body = m.group(1)
                for f in fields:
                    body = body.replace(f"pub {f}: i32,", f"pub {f}: bool,")
                return body
            content = pattern.sub(repl_struct, content)

        # 2. Update assignments
        for f in all_fields:
            content = content.replace(f".{f} = 0 as i32", f".{f} = false")
            content = content.replace(f".{f} = 1 as i32", f".{f} = true")
            # Also handle simple variable assignments if we encounter them, but cautious here
            # content = re.sub(r"\b" + f + r"\s*=\s*0\s*as\s*i32", f + " = false", content)

        # 3. Update checks
        for f in all_fields:
            # field != 0 -> field
            content = re.sub(r"\b" + f + r"\s*!=\s*0(?:\s*as\s*i32)?\b", f, content)
            # field == 1 -> field
            content = re.sub(r"\b" + f + r"\s*==\s*1(?:\s*as\s*i32)?\b", f, content)
            # field == 0 -> !field
            content = re.sub(r"\b" + f + r"\s*==\s*0(?:\s*as\s*i32)?\b", "!" + f, content)
            
            # Handle cases like `if field {` where it was `if field != 0 {`
            # The above regexes should have caught them.

        if orig != content:
            with open(filepath, "w") as f:
                f.write(content)
            count_files += 1
            
    print(f"Booleanified fields in {count_files} files.")

if __name__ == "__main__":
    targets = {
        "VpxInternalErrorInfo": ["has_detail", "setjmp"],
        "Macroblockd": ["corrupted", "up_available", "left_available"],
        "VP8Common": [
            "show_frame", "refresh_last_frame", "refresh_golden_frame", 
            "refresh_alt_ref_frame", "copy_buffer_to_gf", "copy_buffer_to_arf", 
            "refresh_entropy_probs", "mb_no_coeff_skip", "no_lpf", 
            "use_bilinear_mc_filter", "full_pixel"
        ],
        "VpxImage": ["img_data_owner", "self_allocd"],
        "Vp8dComp": [
            "ready_for_new_data", "ec_enabled", "ec_active", 
            "decoded_key_frame", "independent_partitions", 
            "frame_corrupt_residual", "restart_threads"
        ],
        "FragmentData": ["enabled"]
    }
    booleanify(targets)