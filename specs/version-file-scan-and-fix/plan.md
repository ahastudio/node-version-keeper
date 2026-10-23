# Implementation Plan: Version File Scan and Fix

## Summary

Download the fixed version for each major from the official release list, walk the folder, find the positions of version values in five kinds of files, judge them, then report or rewrite only those positions.
This tool is a command-line tool, not a server application, so the layers map to command line and report (UI), analysis and fix application (Application), version and judgement (Domain), and files/network/config (Infrastructure).

## Requirements

1. Find every Node.js version in five kinds of files under the folder. (FR-1)
2. The fixed version is the most recent security release of each major in the official release list. (FR-2)
3. Only a lower exact version is a value to raise to the fixed version of the same major. (FR-3)
4. Without options it does not change files; it fixes only when `--write` is given. (FR-4)
5. Keep the shape of the original value, and report values that are not fixed with a reason. (FR-5, FR-6)
6. Skip `node_modules`, `.git`, and the exclusion list in the config file. (FR-7, FR-8, FR-9, FR-10)
7. Show per-section lists and a summary line, and set the exit code. (FR-11, FR-12, FR-13)

## Critical Files

### New Files

- `Cargo.toml`
- `mise.toml`
- `rustfmt.toml`
- `.gitignore`
- `node-version-keeper-config.json`
- `README.md`
- `src/main.rs`
- `src/lib.rs`
- `src/command.rs`
- `src/command_line.rs`
- `src/report.rs`
- `src/analysis.rs`
- `src/apply.rs`
- `src/observation.rs`
- `src/occurrence.rs`
- `src/verdict.rs`
- `src/judgement.rs`
- `src/version.rs`
- `src/version_text.rs`
- `src/release_catalog.rs`
- `src/http_source.rs`
- `src/detection.rs`
- `src/plain_version_detector.rs`
- `src/tool_versions_detector.rs`
- `src/package_json_detector.rs`
- `src/dockerfile_detector.rs`
- `src/workflow_detector.rs`
- `src/file_scanner.rs`
- `src/config.rs`
- `src/text_file.rs`
- `src/error.rs`
- `tests/common/mod.rs`
- `tests/binary_tests.rs`, `tests/command_line_tests.rs`, `tests/command_tests.rs`, `tests/report_tests.rs`
- `tests/judgement_tests.rs`, `tests/version_tests.rs`, `tests/release_catalog_tests.rs`
- `tests/detector_tests.rs`, `tests/detection_tests.rs`, `tests/file_scanner_tests.rs`, `tests/config_tests.rs`, `tests/http_source_tests.rs`

### Modified Files

- None. It is a new project.

### Reference Files

- `https://nodejs.org/dist/index.json`: Release list. Each entry has `version` and `security`.
- `https://raw.githubusercontent.com/nodejs/Release/main/schedule.json`: Schedule per major

## Architecture

### User Flow

```text
User → [node-version-keeper <folder> (--write)] → parse command line
                                                      ↓
                                      download official release list and schedule
                                                      ↓
                                  walk folder → find version values per file → judge
                                                      ↓
                                      (fix files if --write) → print report
                                                      ↓
                                                  exit code
```

### Event Flow

```text
1. command::run: parses the arguments (with --help it prints usage and ends)
       ↓
2. release_catalog::load_catalog: downloads the release list and schedule and builds the fixed version per major
       ↓
3. file_scanner::collect_candidate_files: collects version files, skipping excluded folders
       ↓
4. analysis::analyze_file: the detector for each file kind finds the value positions and judges them
       ↓
5. apply::apply_all: with --write, rewrites only the values to fix, starting from the later positions
       ↓
6. report::render_report, report::exit_code: decides the report and exit code
```

### Domain Model

```text
NodeVersion (major, minor, patch)
VersionText (Exact | MajorOnly | Other)
Verdict
├── Current              ← nothing to fix
├── Fix(replacement)     ← raise to the fixed version of the same major
└── Report(ReportReason) ← do not fix; report it
Observation (path, occurrence, classification, verdict)
ReleaseCatalog (fixed version per major)
```

## Implementation Steps

### Step 1: UI Layer - Command Line and Report

**Red:** `tests/command_line_tests.rs`, `tests/command_tests.rs`, `tests/report_tests.rs`, `tests/binary_tests.rs`

```text
a_folder_alone_is_a_preview_run, reads_every_option, help_wins_over_everything_else,
rejects_an_unknown_option, rejects_options_missing_their_value, rejects_two_folders,
preview_lists_everything_and_changes_nothing, write_alone_applies_only_same_major_fixes,
a_project_that_is_already_current_exits_zero, exits_zero_when_writing_resolves_everything,
prints_the_usage_for_help, exits_with_code_two_when_no_folder_is_given
```

**Green:** `src/command_line.rs`, `src/command.rs`, `src/report.rs`, `src/main.rs`

**Refactoring:** Reworked parsing to collect options directly into `Options`, and split method bodies to 5 lines or fewer.

### Step 2: Application Layer - Analysis and Fix Application

**Red:** `tests/command_tests.rs`, `tests/judgement_tests.rs`

```text
a_failed_release_download_is_an_error_and_changes_nothing,
a_file_that_cannot_be_written_is_an_error, an_unreadable_version_file_is_an_error,
analyzing_a_file_that_is_not_a_version_file_is_an_error
```

**Green:** `src/analysis.rs`, `src/apply.rs`, `src/observation.rs`

**Refactoring:** None.

### Step 3: Domain Layer - Version and Judgement

**Red:** `tests/version_tests.rs`, `tests/judgement_tests.rs`, `tests/release_catalog_tests.rs`

```text
parses_exact_versions_with_and_without_prefix, classifies_major_only_values,
raises_an_exact_version_below_the_fixed_version_within_its_major,
leaves_an_exact_version_at_or_above_the_fixed_version,
fixed_version_is_the_newest_security_release_of_each_major
```

**Green:** `src/version.rs`, `src/version_text.rs`, `src/verdict.rs`, `src/judgement.rs`, `src/release_catalog.rs`, `src/occurrence.rs`

**Refactoring:** Split the description and value-to-write logic in `src/verdict.rs` into pieces of 5 lines or fewer.

### Step 4: Infrastructure Layer - Files, Network, Config

**Red:** `tests/detector_tests.rs`, `tests/detection_tests.rs`, `tests/file_scanner_tests.rs`, `tests/config_tests.rs`, `tests/http_source_tests.rs`

```text
plain_version_file_reads_the_first_value_line, tool_versions_reads_every_version_on_node_lines,
dockerfile_reads_the_version_part_of_node_image_tags, package_json_reads_engines_and_volta_values,
workflow_reads_scalars_arrays_and_block_lists,
collects_version_files_in_sorted_order_and_skips_excluded_folders,
does_not_follow_symbolic_links, the_default_config_excludes_build_output_folders,
fetches_the_body_of_a_successful_response, a_not_found_response_is_an_error_naming_the_url
```

**Green:** `src/detection.rs`, `src/plain_version_detector.rs`, `src/tool_versions_detector.rs`, `src/package_json_detector.rs`, `src/dockerfile_detector.rs`, `src/workflow_detector.rs`, `src/file_scanner.rs`, `src/config.rs`, `src/http_source.rs`, `src/text_file.rs`, `src/error.rs`

**Refactoring:** Consolidated the conversion that attaches a path to I/O errors into the single function `with_path`.

## Verification

### Build

```bash
mise run build
```

### Test

```bash
mise run test
mise run cover
```

### Manual Test

1. Run without options on a sample folder containing lower versions and check that no file changes.
2. Run with `--write` on the same folder and confirm the file contents changed.
3. Simulate a situation where the official release list cannot be downloaded and check that no file changes.

## Considerations

### Reuse of Existing Code

It is a new project, so no code was reused.
The task names in `mise.toml` are `build`, `test`, and `cover`, and `cover` fails unless line coverage is 100%.

### Compatibility

A config file given with `--config` replaces the default config file rather than merging with it. A hand-made file must also list every folder to exclude.
`node_modules` and `.git` are always excluded, whatever config file is given.
