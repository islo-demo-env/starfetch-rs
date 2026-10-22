# C++ starfetch — quirks and notes for the Rust port

## Hardcoded data path

The C++ binary hardcodes `/usr/local/share/starfetch/` as the constellation data directory (line 31). The parity binary was patched at build time to honor the `STARFETCH_DATA` env var, falling back to the default. The Rust port should accept this env var (or a similar mechanism) and default to the same path.

## ~/.fileToParseWithConstellations.txt shuffle file

When starfetch displays a random constellation (no args, `-r`, or `-c <color>` without `-n`), it uses a shuffle-bag mechanism:
1. On first run (or when the file is empty), it enumerates all `.json` files from both `constellations/` and `norse-constellations/`, shuffles them with `std::mt19937`, writes the list to `~/.fileToParseWithConstellations.txt`.
2. Each run pops the first entry and rewrites the file.
3. When the file is exhausted, it reshuffles.

This means `-r` and no-args output is non-deterministic — golden parity for these cases must be structural only (check box drawing, ANSI codes, field labels, non-empty star graph), not byte-exact.

The corpus.sh harness removes this file before each case to avoid cross-contamination.

## Error handling always exits 0

All error paths (missing constellation name, invalid flag, unrecognized constellation) call `Error()` which prints the error message and then calls `Help()` — which calls `exit(EXIT_SUCCESS)`. So every error case exits 0. The Rust port may choose to exit non-zero for errors, but parity goldens record exit 0 for the `missing` case.

## Fragile argv parsing

The C++ code inspects `argv[1][1]` (the second character of the first argument) to determine the flag. This means:
- `-n` and `--name` both match on `n` at index 1 (but `--name` would set `argv[2]` to the next token, not strip the prefix).
- `-c cyan -n orion` works because it hits the `-c` branch with argc >= 5, where `argv[4]` is taken as the constellation name.
- Single-character check means `-h`, `-help`, `--help` all match the `h` case.
- Unknown flags hit the `default` branch with `argv[1]` shown in the error message.

## -n norse <name> multi-word handling

`-n norse karlvagn` is handled specially: if `argv[2] == "norse"` and `argc >= 4`, it looks in `norse-constellations/` using `argv[3]`. Otherwise it uses the default `constellations/` directory. There's no general "tradition" system — it's literally an `if` for the string `"norse"`.

## -l list ordering

The list output uses `std::filesystem::directory_iterator`, which does NOT guarantee alphabetical order. The listing order may vary across OS/filesystem. The Rust port's `-l` output should match structurally (same set of names) but need not be byte-identical in ordering. The output includes ANSI color codes around each name and the section headers.

The list shows two sections:
1. `✦ available constellations` — all files in `constellations/`
2. `✦ available norse-constellations` — all files in `norse-constellations/`

File extension `.json` is stripped. `.DS_Store` entries are filtered out (macOS artifact check).

## -p flag (legacy)

The `-p` flag exists and behaves identically to `-n` for the default (non-Norse) constellation directory. It's undocumented in the help text.

## -t flag

The `-t` flag sets `useRandomConst = 1` (same as `-r`). Undocumented.

## Template system

The output uses a `template` file in the data directory. It contains placeholders:
- `%0` — title (fullwidth Unicode)
- `%1`–`%10` — star graph lines
- `%11`–`%16` — name, quadrant, right ascension, declination, area, main stars
- `^` characters are replaced with `\033` (ESC) for ANSI codes
- `requestedColor` string is regex-replaced with the chosen color code

## GUI directory

A `GUI/` directory exists in the repo root. It's a Qt-based graphical frontend and is out of scope for the Rust CLI port.

## Color handling

Colors map to ANSI bold codes:
- black: `\033[1;30m`
- white: `\033[1;37m` (default)
- cyan: `\033[1;36m`
- magenta: `\033[1;35m`
- yellow: `\033[1;33m`
- green: `\033[1;32m`
- red: `\033[1;31m`
- blue: `\033[1;34m`

Invalid color names are silently ignored (the default white is kept).

## Star graph rendering

The graph is 22 columns × 10 rows. For each cell, if the JSON specifies a star character at that position, it's wrapped in `REQUESTED_COLOR ... \033[0;0m`. Otherwise a space is output. The graph is rendered left-to-right, top-to-bottom.
