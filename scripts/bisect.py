#!/usr/bin/env python3
"""
bisect.py

A performance bisection script for CrabVPX. It executes the test harness for
the CrabVPX Rust implementation across a commit range to discover performance
regressions by subdividing intervals with the largest performance decrease.

Results are automatically recorded to both out/bisect_results.json and 
out/bisect_results.csv for graphing and post-analysis.

Usage:
  ./scripts/bisect.py --start <starting_commit> [--head <ending_commit>] [--steps <n>]
"""

import argparse
import csv
import json
import re
import subprocess
import sys
from pathlib import Path
from typing import Dict, List

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


def get_commits(start: str, head: str) -> List[str]:
    """Retrieves the chronological list of commit hashes from start to head."""
    try:
        start_hash = subprocess.run(["git", "rev-parse", start], capture_output=True, text=True, check=True).stdout.strip()
        rev_list = subprocess.run(
            ["git", "rev-list", "--topo-order", "--reverse", f"{start_hash}..{head}"],
            capture_output=True, text=True, check=True
        ).stdout.splitlines()
        return [start_hash] + [h.strip() for h in rev_list if h.strip()]
    except subprocess.CalledProcessError as e:
        print(f"Error resolving commits from {start} to {head}: {e.stderr}")
        sys.exit(1)


def measure_commit(commit_hash: str, harness_dir: Path, harness_args: List[str]) -> float:
    """Checks out a commit and benchmarks the CrabVPX Rust implementation."""
    print(f"\n==================================================", flush=True)
    print(f"Checking out commit {commit_hash[:8]}...", flush=True)
    subprocess.run(["git", "checkout", commit_hash], check=True)

    cmd = ["cargo", "+nightly", "run", "--release", "--no-default-features", "--features", "rust", "--"] + harness_args
    print(f"Running benchmark at {commit_hash[:8]}...", flush=True)
    proc = subprocess.run(cmd, cwd=harness_dir, stdout=subprocess.PIPE, text=True, check=True)

    for line in proc.stdout.splitlines():
        if "OVERALL_SUITE_PERF:" in line:
            print(f"Result for {commit_hash[:8]}: {line.strip()}", flush=True)
            match = re.search(r"([0-9.]+)\s*ms/frame", line)
            if match:
                return float(match.group(1))

    print(f"Error: Could not parse OVERALL_SUITE_PERF line from output at commit {commit_hash}")
    sys.exit(1)


def save_results(out_dir: Path, commits: List[str], measured: Dict[int, float]):
    """Records the measured performance profile to both JSON and CSV files."""
    out_dir.mkdir(parents=True, exist_ok=True)
    json_path = out_dir / "bisect_results.json"
    csv_path = out_dir / "bisect_results.csv"

    sorted_indices = sorted(measured.keys())
    
    data = []
    for idx in sorted_indices:
        data.append({
            "index": idx,
            "commit": commits[idx],
            "perf_ms_per_frame": measured[idx]
        })

    # Save JSON
    json_path.write_text(json.dumps(data, indent=2))

    # Save CSV
    with open(csv_path, mode="w", newline="", encoding="utf-8") as f:
        writer = csv.writer(f)
        writer.writerow(["Index", "Commit", "PerfMsPerFrame"])
        for row in data:
            writer.writerow([row["index"], row["commit"], f"{row['perf_ms_per_frame']:.4f}"])

    print(f"Results saved to {json_path} and {csv_path}")


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(description="Bisect to find CrabVPX performance regressions.")
    parser.add_argument("--start", required=True, help="Starting commit (earlier in history).")
    parser.add_argument("--head", default="HEAD", help="Ending commit (default: HEAD).")
    parser.add_argument("-b", "--benchmark", action="store_true", help="Ignored, always runs in benchmark mode.")
    parser.add_argument("-r", "--runs", type=int, default=10, help="Number of benchmark iterations per commit.")
    parser.add_argument("--steps", type=int, default=10, help="Maximum number of bisection steps.")
    parser.add_argument("--out-dir", type=str, default="out", help="Output directory to save bisect_results.json/csv.")
    args, unknown = parser.parse_known_args()

    root_dir = Path(__file__).resolve().parent.parent
    harness_dir = root_dir / "harness"
    test_data_dir = root_dir / "libvpx-test-data"
    out_dir = root_dir / args.out_dir

    download_vectors(test_data_dir)

    harness_args = ["--dir", str(test_data_dir), "--benchmark", "--runs", str(args.runs)]
    harness_args.extend(unknown)

    initial_branch = subprocess.run(["git", "rev-parse", "--abbrev-ref", "HEAD"], capture_output=True, text=True).stdout.strip()
    if initial_branch == "HEAD":
        initial_branch = subprocess.run(["git", "rev-parse", "HEAD"], capture_output=True, text=True).stdout.strip()

    try:
        commits = get_commits(args.start, args.head)
        print(f"Commit range: {len(commits)} commits from {args.start} to {args.head}.")
        if len(commits) < 2:
            print("Error: Commit range too small to bisect.")
            sys.exit(1)

        measured: Dict[int, float] = {}

        # Measure START and HEAD
        measured[0] = measure_commit(commits[0], harness_dir, harness_args)
        measured[len(commits) - 1] = measure_commit(commits[-1], harness_dir, harness_args)
        save_results(out_dir, commits, measured)

        for step in range(args.steps):
            indices = sorted(measured.keys())

            print("\n--- Current Performance Profile ---")
            print(f" {'Idx':<5} | {'Commit':<10} | {'Perf (ms/frame)':<15} | {'Interval Diff (ms)':<18}")
            print("-" * 60)
            for i in range(len(indices)):
                idx = indices[i]
                commit_short = commits[idx][:8]
                perf = measured[idx]
                if i > 0:
                    prev_idx = indices[i-1]
                    diff = perf - measured[prev_idx]
                    interval_str = f"{diff:>+8.2f} ms ({idx - prev_idx} commits)"
                else:
                    interval_str = "-"
                print(f" {idx:<5} | {commit_short:<10} | {perf:>15.2f} | {interval_str}")
            print("-" * 60)

            best_interval = None
            max_diff = -float('inf')

            for i in range(len(indices) - 1):
                a = indices[i]
                b = indices[i+1]
                if b - a > 1:
                    diff = measured[b] - measured[a]
                    if best_interval is None or diff > max_diff:
                        max_diff = diff
                        best_interval = (a, b)

            if not best_interval:
                print("\nNo further commit intervals can be subdivided.")
                break

            a, b = best_interval
            mid = (a + b) // 2
            print(f"\nStep {step+1}/{args.steps}: Selecting midpoint index {mid} between {a} and {b} (diff: {max_diff:+.2f} ms/frame)")
            measured[mid] = measure_commit(commits[mid], harness_dir, harness_args)
            save_results(out_dir, commits, measured)

        # Final output display
        indices = sorted(measured.keys())
        print("\n=== Final Performance Profile ===")
        print(f" {'Idx':<5} | {'Commit':<10} | {'Perf (ms/frame)':<15} | {'Interval Diff (ms)':<18}")
        print("-" * 60)
        for i in range(len(indices)):
            idx = indices[i]
            commit_short = commits[idx][:8]
            perf = measured[idx]
            if i > 0:
                prev_idx = indices[i-1]
                diff = perf - measured[prev_idx]
                interval_str = f"{diff:>+8.2f} ms ({idx - prev_idx} commits)"
            else:
                interval_str = "-"
            print(f" {idx:<5} | {commit_short:<10} | {perf:>15.2f} | {interval_str}")
        print("-" * 60)

    finally:
        print(f"\nRestoring initial git state ({initial_branch})...", flush=True)
        subprocess.run(["git", "checkout", initial_branch], check=True)

    print("")
    tracker_script = root_dir / "scripts" / "count_unsafe.sh"
    if tracker_script.exists():
        subprocess.run([str(tracker_script)], check=True)


if __name__ == "__main__":
    main()
