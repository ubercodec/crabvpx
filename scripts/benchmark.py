#!/usr/bin/env python3
import subprocess
import re
import sys
from pathlib import Path

def run_harness():
    print("Running benchmarking harness...")
    # Call the existing run_harness.sh script with the -b flag
    try:
        result = subprocess.run(
            ["./run_harness.sh", "-b"],
            capture_output=True,
            text=True,
            check=True
        )
        return result.stdout
    except subprocess.CalledProcessError as e:
        print(f"Error running harness: {e}")
        print(e.stdout)
        print(e.stderr)
        sys.exit(1)

def parse_perf_line(line):
    # Example: OVERALL_SUITE_PERF: avg 145.20ms, min 141.50ms, max 149.30ms, 0.12 ms/frame
    match = re.search(r"avg\s+([0-9.]+)(ms|µs|s),\s*min\s+([0-9.]+)(ms|µs|s),\s*max\s+([0-9.]+)(ms|µs|s),\s*([0-9.]+)\s*ms/frame", line)
    if not match:
        return None
        
    def to_ms(val, unit):
        val = float(val)
        if unit == 's': return val * 1000.0
        if unit == 'µs': return val / 1000.0
        return val

    return {
        "avg": to_ms(match.group(1), match.group(2)),
        "min": to_ms(match.group(3), match.group(4)),
        "max": to_ms(match.group(5), match.group(6)),
        "ms_per_frame": float(match.group(7))
    }

def main():
    output = run_harness()
    
    oracle_perf = None
    rust_perf = None
    
    # Simple state machine to parse the output which runs Oracle then Rust
    current_decoder = None
    
    for line in output.splitlines():
        if "Testing C Oracle Decoder" in line:
            current_decoder = "oracle"
        elif "Testing CrabVPX Rust Decoder" in line:
            current_decoder = "rust"
        elif "OVERALL_SUITE_PERF:" in line:
            perf = parse_perf_line(line)
            if current_decoder == "oracle":
                oracle_perf = perf
            elif current_decoder == "rust":
                rust_perf = perf

    if not oracle_perf or not rust_perf:
        print("Error: Could not parse performance metrics from harness output.")
        print(output)
        sys.exit(1)

    # Calculate regression metrics
    # Regression = (Rust_Time - Oracle_Time) / Oracle_Time * 100
    # Positive means Rust is slower (regression), negative means Rust is faster (improvement)
    
    avg_diff = ((rust_perf['avg'] - oracle_perf['avg']) / oracle_perf['avg']) * 100
    min_diff = ((rust_perf['min'] - oracle_perf['min']) / oracle_perf['min']) * 100
    max_diff = ((rust_perf['max'] - oracle_perf['max']) / oracle_perf['max']) * 100
    frame_diff = ((rust_perf['ms_per_frame'] - oracle_perf['ms_per_frame']) / oracle_perf['ms_per_frame']) * 100

    print("==================================================")
    print("🦀 CrabVPX vs Oracle Performance Benchmark 🦀")
    print("==================================================")
    print(f"{'Metric':<15} | {'C Oracle':<10} | {'CrabVPX':<10} | {'Diff (%)':<10}")
    print("-" * 50)
    print(f"{'Suite Average':<15} | {oracle_perf['avg']:>7.2f} ms | {rust_perf['avg']:>7.2f} ms | {avg_diff:>+8.2f} %")
    print(f"{'Suite Min':<15} | {oracle_perf['min']:>7.2f} ms | {rust_perf['min']:>7.2f} ms | {min_diff:>+8.2f} %")
    print(f"{'Suite Max':<15} | {oracle_perf['max']:>7.2f} ms | {rust_perf['max']:>7.2f} ms | {max_diff:>+8.2f} %")
    print(f"{'Per Frame':<15} | {oracle_perf['ms_per_frame']:>7.2f} ms | {rust_perf['ms_per_frame']:>7.2f} ms | {frame_diff:>+8.2f} %")
    print("==================================================")

if __name__ == "__main__":
    main()
