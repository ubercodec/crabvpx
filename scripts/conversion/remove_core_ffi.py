import os, glob, re

def main():
    # Matches `::core::ffi::c_void`, `core::ffi::c_int`, etc.
    core_ffi_pattern = re.compile(r":?core::ffi::([a-zA-Z0-9_]+)")
    count = 0
    
    for filepath in glob.glob("src/**/*.rs", recursive=True):
        with open(filepath, "r") as f:
            content = f.read()
        
        orig = content
        
        # Find all unique ffi types used in this file before replacing
        ffi_types = set(core_ffi_pattern.findall(content))
        
        if ffi_types:
            # Replace instances of `::core::ffi::type` or `core::ffi::type` with just `type`
            content = core_ffi_pattern.sub(r"\1", content)
            
            # Ensure the required `use std::ffi::{...};` import is at the top of the file
            # If we haven't already added `std::ffi` imports, add them.
            if "use std::ffi::" not in content:
                imports_str = f"use std::ffi::{{{', '.join(sorted(ffi_types))}}};"
                
                # Insert the use statement right after any `#![...]` declarations
                lines = content.split("\n")
                insert_idx = 0
                for i, line in enumerate(lines):
                    if not line.startswith("#!"):
                        insert_idx = i
                        break
                lines.insert(insert_idx, imports_str)
                content = "\n".join(lines)
            
            if orig != content:
                with open(filepath, "w") as f:
                    f.write(content)
                count += 1
            
    print(f"Removed core::ffi:: prefixes and added imports to {count} files.")

if __name__ == "__main__":
    main()