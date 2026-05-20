import json
import sys
import os

def parse_log(filename):
    file_frames = {}
    if not os.path.exists(filename):
        print(f"File not found: {filename}")
        return file_frames
    with open(filename, 'r') as f:
        for line in f:
            if line.startswith("FRAME_DATA: "):
                try:
                    data = json.loads(line.replace("FRAME_DATA: ", ""))
                    file_name = data["file"]
                    if file_name not in file_frames:
                        file_frames[file_name] = 0
                    file_frames[file_name] += 1
                except Exception as e:
                    print(f"Error parsing line: {line}, {e}")
                    pass
    return file_frames

# Run from workspace root, so paths are relative to root or absolute
harness_dir = "harness"
oracle = parse_log(os.path.join(harness_dir, "harness_oracle.log"))
rust = parse_log(os.path.join(harness_dir, "harness_rust.log"))

all_files = set(oracle.keys()).union(set(rust.keys()))

print(f"{'File':<40} | {'Oracle':<6} | {'Rust':<6} | {'Diff':<6}")
print("-" * 65)
mismatches = 0
for f in sorted(all_files):
    o_count = oracle.get(f, 0)
    r_count = rust.get(f, 0)
    diff = o_count - r_count
    if diff != 0:
        print(f"{f:<40} | {o_count:<6} | {r_count:<6} | {diff:<6}")
        mismatches += 1

if mismatches == 0:
    print("All frame counts match!")
