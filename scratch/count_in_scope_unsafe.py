import os
import re

out_of_scope = [
    "src/vp8/vp8_dx_iface.rs",
    "src/vpx/src/vpx_encoder.rs",
    "src/vpx/src/vpx_codec.rs",
    "src/vpx/src/vpx_decoder.rs",
    "src/vpx/src/vpx_image.rs",
    "src/api.rs"
]

def is_out_of_scope(path):
    for oos in out_of_scope:
        if path.endswith(oos):
            return True
    return False

unsafe_pattern = re.compile(r'\bunsafe\b')

total_unsafe = 0
in_scope_unsafe = 0

print("Unsafe count per in-scope file:")
for root, dirs, files in os.walk("src"):
    for file in files:
        if file.endswith(".rs"):
            path = os.path.join(root, file)
            with open(path, "r") as f:
                content = f.read()
            count = len(unsafe_pattern.findall(content))
            total_unsafe += count
            if is_out_of_scope(path):
                # print(f"  (OOS) {path}: {count}")
                pass
            else:
                if count > 0:
                    print(f"  {path}: {count}")
                in_scope_unsafe += count

print(f"\nTotal unsafe: {total_unsafe}")
print(f"In-scope unsafe: {in_scope_unsafe}")
print(f"Out-of-scope unsafe: {total_unsafe - in_scope_unsafe}")
