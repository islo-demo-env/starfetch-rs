# Parity Report: Rust starfetch vs C++ goldens

**Date:** 2026-10-22
**Branch:** port/rust-starfetch
**Rust binary:** /workspace/repo/starfetch/target/debug/starfetch
**Goldens:** /workspace/.parity/goldens/
**Rust outputs:** /workspace/.parity/runs/rust/

## Environment

- Rust toolchain: rustc 1.99.1 (stable)
- Build: `cargo build` with `CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=clang` (wrapper → gcc-12 + bundled lld)
- Data: `STARFETCH_DATA=/workspace/repo/res/`

## Deterministic Cases (byte-exact parity required)

| Case | Args | Exit Code | Output | Verdict |
|---|---|---|---|---|
| help | `-h` | MATCH (0/0) | BYTE-EXACT | **PASS** |
| orion | `-n orion` | MATCH (0/0) | BYTE-EXACT | **PASS** |
| norse-karlvagn | `-n norse karlvagn` | MATCH (0/0) | BYTE-EXACT | **PASS** |
| orion-cyan | `-c cyan -n orion` | MATCH (0/0) | BYTE-EXACT | **PASS** |
| missing | `-n missing_name` | MATCH (0/0) | BYTE-EXACT | **PASS** |
| colors-only | `-c` | MATCH (0/0) | BYTE-EXACT | **PASS** |

## Partially Deterministic Cases

| Case | Args | Exit Code | Output | Verdict |
|---|---|---|---|---|
| list | `-l` | MATCH (0/0) | BYTE-EXACT | **PASS** |

Note: list output was byte-exact in this run. Per corpus.md, filesystem iteration order may vary across environments; structural match (same set of constellation names) is the minimum bar.

## Non-Deterministic Cases (structural parity only)

These cases involve random constellation selection. Structural checks verify: exit code 0, box-drawing characters present (│ ×10 = template frame), and all data fields (Quadrant, Right ascension, Declination, Area, Main stars) are present.

| Case | Args | Exit Code | Structure | Size | Verdict |
|---|---|---|---|---|---|
| no-args | *(none)* | MATCH (0/0) | All fields present, box frame intact | 754 B | **PASS** |
| random | `-r` | MATCH (0/0) | All fields present, box frame intact | 727 B | **PASS** |
| color-random | `-c cyan` | MATCH (0/0) | All fields present, box frame intact | 739 B | **PASS** |

## Exit Code Summary

All 10 cases exit 0, matching C++ goldens. No exit code mismatches.

## Diff Paths

No diffs generated — all deterministic cases are byte-exact. Rust outputs are at `/workspace/.parity/runs/rust/`.

## Summary

- **Deterministic cases:** 6/6 PASS (byte-exact)
- **Partially deterministic (list):** 1/1 PASS (byte-exact)
- **Structural (random):** 3/3 PASS
- **Total:** 10/10 PASS

## Verdict: PASSED
