#!/bin/bash
# scripts/analyze_complexity.sh
# Analyzes the complexity of the c2rust transpiled files by counting unsafe usage.

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SRC_DIR="${DIR}/src"
OUT_DIR="${DIR}/out"

mkdir -p "$OUT_DIR"
OUT_FILE="$OUT_DIR/unsafe_analysis.md"

cd "$DIR"

TOTAL_BLOCKS=$(grep -r "unsafe {" src/ | wc -l | tr -d ' ')
TOTAL_FNS=$(grep -r -E "unsafe.*fn" src/ | wc -l | tr -d ' ')

{
    echo "### Unsafe Usage Analysis"
    echo "A programmatic analysis of the \`c2rust\` generated codebase reveals the following scale of unsafety:"
    echo ""
    echo "- **Total Unsafe Blocks (\`unsafe { ... }\`):** $TOTAL_BLOCKS"
    echo "- **Total Unsafe Functions (\`unsafe fn\`):** $TOTAL_FNS"
    echo ""
    echo "#### Top 10 Most Complex Files (by Unsafe Blocks)"
    echo "| File | Unsafe Blocks |"
    echo "|---|---|"
    grep -rc "unsafe {" src/ | awk -F: '$2 > 0 {print $2, $1}' | sort -nr | head -n 10 | while read -r count file; do
        echo "| \`$file\` | $count |"
    done
    echo ""
    echo "#### Top 10 Most Complex Files (by Unsafe Functions)"
    echo "| File | Unsafe Functions |"
    echo "|---|---|"
    grep -rc -E "unsafe.*fn" src/ | awk -F: '$2 > 0 {print $2, $1}' | sort -nr | head -n 10 | while read -r count file; do
        echo "| \`$file\` | $count |"
    done
    echo ""
    echo "These files represent the highest risk and effort areas for manual refactoring to safe Rust."
} > "$OUT_FILE"

echo "Analysis complete. Output written to $OUT_FILE"
