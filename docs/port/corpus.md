# Test corpus for starfetch parity

Each case is run by `corpus.sh <binary> <outdir>`, producing `<case>.out` and `<case>.exit`.

The `STARFETCH_DATA` env var must point to the constellation data directory (with trailing `/`). For the C++ goldens, this is `/workspace/repo/res/`.

## Deterministic cases (byte-exact parity expected)

| Case | Args | Byte-exact? | Notes |
|---|---|---|---|
| `help` | `-h` | Yes | Reads `help_message.txt` from data dir. Exit 0. |
| `orion` | `-n orion` | Yes | Standard constellation, default white color. |
| `norse-karlvagn` | `-n norse karlvagn` | Yes | Norse constellation via multi-arg syntax. |
| `orion-cyan` | `-c cyan -n orion` | Yes | Color override, specific constellation. |
| `missing` | `-n missing_name` | Yes | Error message + help text. Exit 0 (not 1 — see NOTES.md). |

## Partially deterministic cases

| Case | Args | Byte-exact? | Notes |
|---|---|---|---|
| `list` | `-l` | Structural | Same set of constellation names, but filesystem iteration order may vary. Two sections: regular and Norse. ANSI color codes around names. |

## Non-deterministic cases (structural parity only)

These cases involve random constellation selection. The golden output is captured but parity checks should verify structure only: box-drawing characters present, ANSI escape sequences, labeled data fields (Name, Quadrant, Right ascension, Declination, Area, Main stars), non-empty star graph region.

| Case | Args | Byte-exact? | Notes |
|---|---|---|---|
| `no-args` | *(none)* | Structural | Random constellation, white color. Uses shuffle-bag file. |
| `random` | `-r` | Structural | Random constellation, white color. |
| `colors-only` | `-c` | Yes | Prints "Available colors are: ..." and exits 0. No constellation shown. |
| `color-random` | `-c cyan` | Structural | Random constellation in cyan. |

## Exit codes

All cases exit 0, including error cases. The C++ code calls `exit(EXIT_SUCCESS)` from the `Help()` function, which is called by all error paths.

## Environment requirements

- `STARFETCH_DATA` — path to data directory (must end with `/`)
- `HOME` — used for the shuffle-bag file `~/.fileToParseWithConstellations.txt`
- The corpus.sh script removes the shuffle-bag file before each case run to avoid cross-contamination.
