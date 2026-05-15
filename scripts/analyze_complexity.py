#!/usr/bin/env python3
"""
analyze_complexity.py

This script serves as both a complexity analyzer for existing c2rust transpiled
Rust code, and an automated pipeline to simulate a full port of libvpx to Rust.

Usage:
  # Analyze an existing directory (like our current src/)
  python3 scripts/analyze_complexity.py --src-dir src/

  # Run the full pipeline (clone, configure, c2rust, analyze) in out/
  python3 scripts/analyze_complexity.py --full-pipeline
"""

import argparse
import logging
import multiprocessing
import re
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Dict, List, Tuple

logging.basicConfig(level=logging.INFO, format="%(levelname)s: %(message)s")

def run_cmd(cmd: List[str], cwd: Path, env: Dict[str, str] = None) -> None:
    """Runs a shell command and checks for success."""
    logging.info(f"Running command: {' '.join(cmd)}")
    subprocess.run(cmd, cwd=cwd, env=env, check=True)

def clone_libvpx(out_dir: Path) -> Path:
    """Clones the libvpx repository into the specified directory."""
    libvpx_path = out_dir / "libvpx_full"
    if libvpx_path.exists():
        logging.info(f"Directory {libvpx_path} already exists, skipping clone.")
        return libvpx_path

    logging.info(f"Cloning libvpx into {libvpx_path}...")
    run_cmd(["git", "clone", "https://chromium.googlesource.com/webm/libvpx", str(libvpx_path)], cwd=out_dir)
    return libvpx_path

def configure_libvpx(libvpx_path: Path) -> None:
    """Configures libvpx with a wide set of features to maximize code coverage."""
    logging.info("Configuring libvpx with multiple features enabled...")
    config_cmd = [
        "./configure",
        "--enable-vp8",
        "--enable-vp9",
        "--enable-vp9-highbitdepth",
        "--enable-postproc",
        "--enable-multithread",
        "--enable-experimental",
        "--enable-spatial-svc",
    ]
    run_cmd(config_cmd, cwd=libvpx_path)

def generate_compile_commands(libvpx_path: Path) -> None:
    """Uses 'bear' to generate compile_commands.json during the build."""
    logging.info("Building libvpx and generating compile_commands.json using bear...")
    if not shutil.which("bear"):
        logging.error("The 'bear' tool is required to generate compile_commands.json but was not found.")
        sys.exit(1)
        
    num_cores = str(multiprocessing.cpu_count())
    run_cmd(["bear", "--", "make", "-j", num_cores], cwd=libvpx_path)
    
    if not (libvpx_path / "compile_commands.json").exists():
        logging.error("Failed to generate compile_commands.json")
        sys.exit(1)

def run_c2rust(libvpx_path: Path) -> None:
    """Runs c2rust transpile on the generated compile_commands.json."""
    logging.info("Transpiling C code to Rust using c2rust...")
    if not shutil.which("c2rust"):
        logging.error("The 'c2rust' tool is required but was not found.")
        sys.exit(1)
        
    run_cmd(["c2rust", "transpile", "compile_commands.json", "--emit-build-files"], cwd=libvpx_path)

def count_unsafe(file_path: Path) -> Tuple[int, int]:
    """
    Counts the number of 'unsafe {' blocks and 'unsafe fn' declarations in a Rust file.
    Returns a tuple of (unsafe_blocks_count, unsafe_functions_count).
    """
    blocks = 0
    fns = 0
    try:
        content = file_path.read_text(encoding='utf-8', errors='replace')
        blocks = len(re.findall(r"unsafe\s*\{", content))
        fns = len(re.findall(r"unsafe\s+(?:extern\s+\"[^\"]*\"\s+)?fn\s+", content))
    except Exception as e:
        logging.warning(f"Failed to read {file_path}: {e}")
    return blocks, fns

def analyze_complexity(src_dir: Path, out_file: Path) -> None:
    """
    Walks through all .rs files in the given directory, counts unsafe usage,
    and generates a detailed Markdown report.
    """
    logging.info(f"Analyzing Rust files in {src_dir} for unsafe usage...")
    
    total_blocks = 0
    total_fns = 0
    file_stats = []
    
    for path in src_dir.rglob("*.rs"):
        if "target" in path.parts or path.name == "build.rs" or path.name == "lib.rs":
            continue
            
        blocks, fns = count_unsafe(path)
        if blocks > 0 or fns > 0:
            total_blocks += blocks
            total_fns += fns
            
            try:
                rel_path = path.relative_to(src_dir)
            except ValueError:
                rel_path = path
                
            file_stats.append((str(rel_path), blocks, fns))
            
    file_stats_by_blocks = sorted(file_stats, key=lambda x: x[1], reverse=True)
    file_stats_by_fns = sorted(file_stats, key=lambda x: x[2], reverse=True)
    
    logging.info("Generating Markdown report...")
    
    report = [
        "# Unsafe Usage Analysis Report\n",
        "This report provides a programmatic analysis of the `c2rust` generated codebase,",
        "quantifying the scale of unsafety and technical debt required for a full safe Rust port.\n",
        f"- **Target Directory:** `{src_dir.name}`",
        f"- **Total Unsafe Blocks (`unsafe {{ ... }}`):** {total_blocks}",
        f"- **Total Unsafe Functions (`unsafe fn`):** {total_fns}\n",
        "## Top 10 Most Complex Files (by Unsafe Blocks)",
        "| File | Unsafe Blocks |",
        "|---|---|"
    ]
    
    for stat in file_stats_by_blocks[:10]:
        report.append(f"| `{stat[0]}` | {stat[1]} |")
    report.append("")
    
    report.extend([
        "## Top 10 Most Complex Files (by Unsafe Functions)",
        "| File | Unsafe Functions |",
        "|---|---|"
    ])
    
    for stat in file_stats_by_fns[:10]:
        report.append(f"| `{stat[0]}` | {stat[2]} |")
    report.append("")
    
    report.extend([
        "## Conclusion",
        "Files with the highest concentration of unsafe blocks typically contain dense pointer",
        "arithmetic, manual memory management, or complex hardware intrinsic calls. These files",
        "represent the highest risk and effort areas for manual refactoring to idiomatic, safe Rust."
    ])
    
    out_file.parent.mkdir(parents=True, exist_ok=True)
    out_file.write_text("\n".join(report))
    logging.info(f"Report written successfully to {out_file}")

def main():
    parser = argparse.ArgumentParser(description="Analyze c2rust transpilation complexity.")
    parser.add_argument("--src-dir", type=Path, help="Specific directory of Rust files to analyze (e.g. 'src/')")
    parser.add_argument("--full-pipeline", action="store_true", help="Run the full pipeline: clone libvpx, c2rust, and analyze")
    parser.add_argument("--out-dir", type=Path, default=Path("out"), help="Output directory for reports and clones (default: 'out')")
    
    args = parser.parse_args()
    
    if not args.src_dir and not args.full_pipeline:
        parser.error("You must specify either --src-dir or --full-pipeline.")
        
    out_dir = args.out_dir.absolute()
    
    if args.full_pipeline:
        logging.info("Starting full libvpx transpilation pipeline simulation...")
        out_dir.mkdir(parents=True, exist_ok=True)
        
        libvpx_path = clone_libvpx(out_dir)
        configure_libvpx(libvpx_path)
        generate_compile_commands(libvpx_path)
        run_c2rust(libvpx_path)
        
        report_file = out_dir / "full_libvpx_unsafe_analysis.md"
        analyze_complexity(libvpx_path, report_file)
        
        logging.info("Pipeline complete.")
        
    elif args.src_dir:
        src_dir = args.src_dir.absolute()
        if not src_dir.exists() or not src_dir.is_dir():
            logging.error(f"Source directory {src_dir} does not exist.")
            sys.exit(1)
            
        report_file = out_dir / "unsafe_analysis.md"
        analyze_complexity(src_dir, report_file)

if __name__ == "__main__":
    main()
