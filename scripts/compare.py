#!/usr/bin/env python3
"""
compare.py

The primary orchestration script for CrabVPX testing. It handles:
1. Downloading the required VP8 test vectors and MD5 reference files.
2. Running differential testing (Oracle vs Rust).
3. Supporting side-by-side direct comparison between implementations.

Usage:
  ./scripts/compare.py [-b/--benchmark] [--side-by-side]
"""

import argparse
import json
import subprocess
import sys
from pathlib import Path
from typing import Dict, List, Optional

# VP8 Comprehensive Test Vectors from WebM Project
VP8_VECTORS = [
    "vp80-00-comprehensive-001.ivf", "vp80-00-comprehensive-002.ivf",
    "vp80-00-comprehensive-003.ivf", "vp80-00-comprehensive-004.ivf",
    "vp80-00-comprehensive-005.ivf", "vp80-00-comprehensive-006.ivf",
    "vp80-00-comprehensive-007.ivf", "vp80-00-comprehensive-008.ivf",
    "vp80-00-comprehensive-009.ivf", "vp80-00-comprehensive-010.ivf",
    "vp80-00-comprehensive-011.ivf", "vp80-00-comprehensive-012.ivf",
    "vp80-00-comprehensive-013.ivf", "vp80-00-comprehensive-014.ivf",
    "vp80-00-comprehensive-015.ivf", "vp80-00-comprehensive-016.ivf",
    "vp80-00-comprehensive-017.ivf", "vp80-00-comprehensive-018.ivf",
    "vp80-01-intra-1400.ivf", "vp80-01-intra-1411.ivf",
    "vp80-01-intra-1416.ivf", "vp80-01-intra-1417.ivf",
    "vp80-02-inter-1402.ivf", "vp80-02-inter-1412.ivf",
    "vp80-02-inter-1418.ivf", "vp80-02-inter-1424.ivf",
    "vp80-03-segmentation-01.ivf", "vp80-03-segmentation-02.ivf",
    "vp80-03-segmentation-03.ivf", "vp80-03-segmentation-04.ivf",
    "vp80-03-segmentation-1401.ivf", "vp80-03-segmentation-1403.ivf",
    "vp80-03-segmentation-1407.ivf", "vp80-03-segmentation-1408.ivf",
    "vp80-03-segmentation-1409.ivf"
]

BASE_URL = "https://storage.googleapis.com/downloads.webmproject.org/test_data/libvpx"


def download_vectors(test_data_dir: Path):
    """Ensures all required VP8 test vectors and MD5s are present."""
    test_data_dir.mkdir(parents=True, exist_ok=True)
    print(f"Checking for VP8 test vectors in {test_data_dir}...", flush=True)

    for vector in VP8_VECTORS:
        for ext in ["", ".md5"]:
            file_name = vector + ext
            target = test_data_dir / file_name
            if not target.exists():
                print(f"Downloading {file_name}...", flush=True)
                url = f"{BASE_URL}/{file_name}"
                subprocess.run(["curl", "-S", "-s", "-L", "-o", str(target), url], check=True)


def run_harness_capture(harness_dir: Path, args: List[str], features: str = None) -> List[Dict]:
    """Runs a cargo command and captures its frame data output."""
    cmd = ["cargo", "run", "--release"]
    if features:
        cmd.extend(["--no-default-features", "--features", features])
    cmd.append("--")
    cmd.extend(args)

    proc = subprocess.run(cmd, cwd=harness_dir, capture_output=True, text=True, check=True)
    
    frames = []
    for line in proc.stdout.splitlines():
        if line.startswith("FRAME_DATA: "):
            frames.append(json.loads(line.replace("FRAME_DATA: ", "")))
    return frames


def run_harness_stream(harness_dir: Path, args: List[str], features: str = None):
    """Runs cargo and streams output to console."""
    cmd = ["cargo", "run", "--release"]
    if features:
        cmd.extend(["--no-default-features", "--features", features])
    cmd.append("--")
    cmd.extend(args)
    subprocess.run(cmd, cwd=harness_dir, check=True)


def perform_side_by_side(harness_dir: Path, harness_args: List[str]):
    """Runs both decoders and compares their frame-by-frame output directly."""
    print("\n--- Running Side-by-Side Direct Comparison ---")
    
    print("Capturing C Oracle output...", flush=True)
    oracle_frames = run_harness_capture(harness_dir, harness_args)
    
    print("Capturing CrabVPX Rust output...", flush=True)
    rust_frames = run_harness_capture(harness_dir, harness_args, features="rust")
    
    if len(oracle_frames) != len(rust_frames):
        print(f"Error: Frame count mismatch! Oracle: {len(oracle_frames)}, Rust: {len(rust_frames)}")
        sys.exit(1)
        
    mismatches = 0
    for i, (o, r) in enumerate(zip(oracle_frames, rust_frames)):
        if o != r:
            print(f"Mismatch in {o['file']} frame {o['idx']}:")
            print(f"  Oracle: {o}")
            print(f"  Rust:   {r}")
            mismatches += 1
            
    if mismatches == 0:
        print(f"✅ Success! Direct comparison passed for all {len(oracle_frames)} frames.")
    else:
        print(f"❌ Failed! Found {mismatches} mismatches.")
        sys.exit(1)


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(description="Run the CrabVPX test harness.")
    parser.add_argument("-b", "--benchmark", action="store_true", help="Run in benchmark mode.")
    parser.add_argument("-r", "--runs", type=int, default=50, help="Number of benchmark iterations.")
    parser.add_argument("-s", "--side-by-side", action="store_true", help="Directly compare implementations.")
    args, unknown = parser.parse_known_args()

    root_dir = Path(__file__).resolve().parent.parent
    harness_dir = root_dir / "harness"
    test_data_dir = root_dir / "libvpx-test-data"

    download_vectors(test_data_dir)

    harness_args = ["--dir", str(test_data_dir)]
    if args.benchmark:
        harness_args.extend(["--benchmark", "--runs", str(args.runs)])
    harness_args.extend(unknown)

    if args.side_by_side:
        perform_side_by_side(harness_dir, harness_args)
    else:
        print("\n--- Testing C Oracle Decoder ---", flush=True)
        run_harness_stream(harness_dir, harness_args)

        print("\n--- Testing CrabVPX Rust Decoder ---", flush=True)
        run_harness_stream(harness_dir, harness_args, features="rust")

    print("")
    tracker_script = root_dir / "scripts" / "count_unsafe.sh"
    if tracker_script.exists():
        subprocess.run([str(tracker_script)], check=True)


if __name__ == "__main__":
    main()
