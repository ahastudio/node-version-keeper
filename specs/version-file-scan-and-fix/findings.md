# Findings & Decisions

> **Update this file immediately whenever there is a technical finding or an important decision.**

## Requirements

- Given one folder, find the Node.js version files of all projects under it.
- Fetch the fixed version automatically from the official release list.
- The default is preview; `--write` fixes.
- Excluded folders are split into those always excluded and those set in the config file.

## Research Findings

### Codebase Structure

- It is a new project, made up of `Cargo.toml`, `mise.toml`, `src/`, `tests/`, and `node-version-keeper-config.json`.
- There is a separate detector for each file kind that finds where the value is. Every detector returns byte positions within the file content.

### Existing Patterns

- Entries of the official release list `index.json` have `version`, `date`, and `security` fields, and the newest entry comes first.
- The official schedule `schedule.json` has `start`, `lts`, `maintenance`, and `end` dates for each major. Lines that are not LTS have no `lts`.
- Rust has no LTS line. So `mise.toml` pins it to a minor version (`1.99`).
- Similar tools were found, but none that sweeps one folder and checks and fixes six kinds of files at once. Renovate's nvm manager keeps `.nvmrc` up to date, but it is configured per repository and delivers changes as pull requests.

## Resources

### Documentation

- [Node.js release list](https://nodejs.org/dist/index.json)
- [Node.js release schedule](https://raw.githubusercontent.com/nodejs/Release/main/schedule.json)
- [Renovate nvm manager](https://docs.renovatebot.com/modules/manager/nvm/)

### Code References

- Computing the fixed version per major from the official release list: `src/release_catalog.rs`
- Judging a single value: `src/judgement.rs`
- File kind classification and detector wiring: `src/detection.rs`

### API Endpoints

- GET `https://nodejs.org/dist/index.json`
- GET `https://raw.githubusercontent.com/nodejs/Release/main/schedule.json`

## Technical Decisions

| Decision | Rationale |
| -------- | --------- |
| Get the fixed version only automatically from the official release list | New security releases need no change to the tool. We decided not to provide an option to specify it directly |
| Running without options is a preview; `--write` fixes | It fixes files in many projects at once, and in a folder without git there may be no way to undo |
| Raise only exact versions to the fixed version of the same major | Changing a major-only value, a range, or an alias would change its meaning |
| Always exclude only `node_modules` and `.git`; set the rest in a config file | The goal is to miss nothing, so default exclusions are kept to a minimum. Excluding hidden folders wholesale would stop `.github/workflows` from being read |
| Embed the default config file in the tool at build time | The default exclusion list applies even with only the executable |
| Get the byte position of the value in the file and rewrite only that spot | Leaves the rest of the file's content and shape as is |
| The fixed version of the same major is the most recent release of that major with `security` set to `true` | Basing it on security releases is the point of the request |
| Use Rust as the implementation language | The user chose Rust after comparing it with Go |
| Consolidate error conversion into a single `with_path` function | Placing conversion closures at every call site showed up as missed lines in line coverage |

## Issues Encountered

### 1. Two compile errors

**Problem**:

On the first build, `src/release_catalog.rs` gave an error that the type could not be determined (E0282), and `src/package_json_detector.rs` gave an error about borrowing a non-mutable value (E0596).

**Cause**:

The first file converted a generic return value once more with `.into_iter().collect()`, so the type could not be determined. The second declared the iterator calling `find_map` without `mut`.

**Resolution**:

The first file now returns the two sets as they are, and the second was fixed to `let mut pairs`.

**Result**:

The build passed.

### 2. Missed lines in line coverage

**Problem**:

`mise run cover` failed with 5 missed lines: 2 in `src/file_scanner.rs`, 1 in `src/report.rs`, and 2 in `src/verdict.rs`.

**Cause**:

No test called the `Verdict::Current` description in `report.rs` or the `ReportReason::EndOfSupport` description in `verdict.rs`. The 2 lines in `file_scanner.rs` were lines with closures that convert I/O errors, and there was no way to trigger those errors in tests.

**Resolution**:

Made `describe` public and added `tests/report_tests.rs`, which checks every verdict and every reason directly. The conversion that attaches a path to I/O errors was consolidated into the single function `with_path` in `src/error.rs`.

**Result**:

0 of 790 lines were missed.

### 3. Method bodies over 5 lines

**Problem**:

Seven functions had bodies longer than 5 lines: 2 in `alignment.rs`, 1 in `command_line.rs`, 2 in `report.rs`, and 2 in `verdict.rs`.

**Cause**:

`match` and struct literals were spread over several lines after `rustfmt`.

**Resolution**:

Split the functions by unit of meaning. Command-line parsing was changed to collect options directly into `Options`.

**Result**:

No function is longer than 5 lines.

### 4. clippy error and warning

**Problem**:

`cargo clippy --all-targets` reported an error for a call in `tests/http_source_tests.rs` that did not use the number of bytes read, and a warning for an unused function in `tests/judgement_tests.rs`.

**Cause**:

The read length was not captured in a variable, and a test helper function that was no longer used was left behind.

**Resolution**:

Captured the read length in a variable and deleted the unused function.

**Result**:

`clippy` has no warnings.

### 5. Wrong test expectation

**Problem**:

`config_option_replaces_the_default_exclusions_but_not_the_fixed_ones` failed because it expected `99.0.0` to appear in the result.

**Cause**:

`99` is a major with no known security release, so it is judged "nothing to fix" and does not appear in the report.

**Resolution**:

Changed the sample value under the excluded folder to the end-of-support `18.20.4`, and checked by whether the paths (`/dist/.nvmrc`, `node_modules`) are in the result rather than by the value.

**Result**:

The test passed.

## Learnings

### Check values that do not appear in the report by path, not by value (2026-10-22)

A value judged "nothing to fix" does not appear in the report.
Whether an excluded folder was applied must be checked by whether that folder's file paths are in the result.

### Consolidate error conversion into one function (2026-10-22)

Placing a closure that attaches a path to I/O errors at every call line showed up as missed lines in line coverage, and consolidating it into the single function `with_path` made them go away.
