# VP8 Decoder Implementation Plan (TDD Execution)

## Objective
You are an autonomous Software Engineering Agent. Your goal is to convert this transpiled C-to-Rust implementation of a vp8 decoder into one that is idiomatic and memory-safe, following the design in safety/architecture_overview.md .

## The Execution Strategy (Test-Driven Development)
This project is strictly governed by TDD. You will not write the entire decoder at once.

**Your Job** is to improve the bitstream parser by making one more symbol parse correctly.  Once you have done this, stop.  If you uncover other bugs along the way, you may try to fix them or document them in an md file.  What you want to avoid is trying to fix 'the next mismatch' after the one you start on; that's a rabbit hole since there are basically unlimited symbols.  Remember that we will eventually must fix them all, so don't take hacky shortcuts that make it harder later on.

**IMPORTANT** NO HACKS!  We want a general-purpose decoder when we're doing; do  not hardcode things that are only specific to our test case!

## Current State
*   **Test Vectors:** park_joy_90p_10_420 and test_1 are our two test vectors.  the files are: `test_vectors/park_joy_90p_10_420/entropy.log` (ground truth) and `test_vectors/park_joy_90p_10_420/obu` and similar.
Test vectors contain TR_SYMBOL and TR_UPDATE lines that are matched by the test harness (see below).  They also contain DEBUG_ lines to show the intermediate state of the decoder, with file and line information that you can look up in ./avm -- this is the reference C decoder.  TRACE_SPEC.md explains the entropy log in more detail, but TR_ lines must match and DEBUG_ lines are just helpful bits of info you can use use to see what the AV2 C reference decoder was thinking.

*   **Test Harness:** `cd av2_decoder && cargo test --test ldd_test`. The harness now parses the `name`, `rng`, and `dif` fields. It immediately panics on the very first mismatch to enforce **Strict Label Parity** and arithmetic sync.
    **CDF Tables** If the CDF seems to mismatch, check the rates against avm first.  The default probabilities were extracted seemingly reliably, but there have been several instances of incorrect rates.
    **Hints**  HINTS.md has some notes from previous instances of you that might be of use (or not - they are not human audited so your mileage may vary)
    **IMPORTANT** park_joy currently passes to eof!  Be sure not to regress it!

Here is an overview of the directory:
* ./src - the Rust decoder ** This is what you are working on **
* ./safety/ - the overall picture of the rust decoder design you are working towards
* ./build.sh - a handy script to build the decoder.  It is basically `cargo build` with some extra flags, since it must be cross-compiled to arm.

## Critical Rules of Engagement
*   **No Unsafe Rust:** You are forbidden from using `unsafe` blocks, `RefCell`, or interior mutability hacks to bypass the borrow checker.

**Your First Action:** When you are initialized in this workspace, acknowledge these instructions, confirm you have read the `architecture_overview.md`, and run the build script to confirm that it builds or not.  If not, fix the first error (working towards our goal of memory safety!) and stop.  If it already builds, pick an unsafe block and try to make it sfe.  IMPORTANT: This work will be done by other agents, too, so please leave instructions for them about where you left off in safety/HINTS.md .  Include information that might help future agents progress further and/or clean up out-of-date previous hints.
