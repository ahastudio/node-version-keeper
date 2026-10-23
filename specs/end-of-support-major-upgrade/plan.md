# Implementation Plan: Major Upgrade for End-of-Support Lines

## Summary

Extract the majors past their end date and the supported LTS majors from the official schedule file, and judge values on end-of-support lines as values to upgrade to the patch version of the supported LTS line above.
Files are written only when the run mode allows it.

## Requirements

1. Treat a major whose end date is before today (UTC) as an end-of-support line. (FR-1)
2. Judge values that pin an exact version or only a major of an end-of-support line as upgrade candidates. (FR-2)
3. The target line is the supported LTS line immediately above and must have a patch version. (FR-3, FR-4)
4. If there is no target line, only report. (FR-5)
5. Write only when `--write` and `--upgrade-major` are both present, and handle the one-sided cases as specified. (FR-6, FR-7)
6. Place the "End of support" section at the top. (FR-8)

## Critical Files

### New Files

- `src/calendar.rs`
- `tests/calendar_tests.rs`

### Modified Files

- `src/release_catalog.rs`: Reads `lts` and `end` from the schedule file to build the end-of-support majors and supported LTS majors, and provides `upgrade_target`
- `src/judgement.rs`: Judges values on end-of-support lines as values to upgrade
- `src/verdict.rs`: `MajorUpgrade`, `UpgradeCause`, `ApplyMode`, and the reason `EndOfSupport`
- `src/command_line.rs`: `--upgrade-major` option and the error when given without `--write`
- `src/report.rs`: "End of support" section and apply status text
- `tests/release_catalog_tests.rs`, `tests/judgement_tests.rs`, `tests/command_line_tests.rs`, `tests/command_tests.rs`, `tests/report_tests.rs`

### Reference Files

- `https://raw.githubusercontent.com/nodejs/Release/main/schedule.json`: Per-major `lts` and `end` dates

## Architecture

### User Flow

```text
User → [node-version-keeper <folder> --write --upgrade-major] → parse command line
                                                                    ↓
                                                  fetch schedule file → compute end-of-support and LTS majors
                                                                    ↓
                                          judge values (on an end-of-support line, the patch version of the target line)
                                                                    ↓
                                          write files only when --write and --upgrade-major are both present
```

### Event Flow

```text
1. command_line::parse_arguments: error if --upgrade-major is present without --write
       ↓
2. release_catalog::load_catalog: build end-of-support majors and supported LTS majors from the schedule file
       ↓
3. judgement::judge: on an end-of-support line, pick the value to upgrade to via upgrade_target
       ↓
4. apply::apply_all: write that value only when ApplyMode allows the major upgrade
       ↓
5. report::render_report: write the apply status in the "End of support" section
```

### Domain Model

```text
Verdict
├── MajorUpgrade { replacement, cause }  ← value to upgrade to and its cause
│   └── cause: UpgradeCause::EndOfSupport
└── Report(ReportReason::EndOfSupport)   ← when there is no target line
ApplyMode { write, upgrade_major }
└── allows_major_upgrade()               ← true only when both are on
ReleaseCatalog
├── is_end_of_support(major)
└── upgrade_target(major)                ← patch version of the nearest supported LTS above
```

## Implementation Steps

### Step 1: UI Layer - Option and report section

**Red:** `tests/command_line_tests.rs`, `tests/report_tests.rs`, `tests/command_tests.rs`

```text
upgrading_majors_requires_write, describes_a_major_upgrade_by_mode,
only_applies_what_the_mode_allows, write_alone_applies_only_same_major_fixes,
write_with_upgrade_major_applies_everything_it_can
```

**Green:** `src/command_line.rs`, `src/report.rs`

**Refactoring:** Split `describe` into `describe_upgrade` and `describe_without_replacement`.

### Step 2: Application Layer - Writing by apply mode

**Red:** `tests/command_tests.rs`

```text
write_alone_applies_only_same_major_fixes,
write_with_upgrade_major_applies_everything_it_can
```

**Green:** `src/apply.rs`

**Refactoring:** None.

### Step 3: Domain Layer - End-of-support judgment and target line selection

**Red:** `tests/judgement_tests.rs`, `tests/release_catalog_tests.rs`

```text
moves_an_end_of_support_exact_version_to_the_nearest_supported_lts,
moves_an_end_of_support_major_only_value_keeping_its_shape,
reports_an_end_of_support_line_when_no_supported_lts_is_above,
upgrade_target_is_the_nearest_supported_lts_above,
upgrade_target_skips_lts_lines_that_have_not_started_or_have_no_fixed_version
```

**Green:** `src/judgement.rs`, `src/verdict.rs`, `src/release_catalog.rs`

**Refactoring:** Split `ReportReason::describe` and `Verdict::applied_replacement` to 5 lines or fewer.

### Step 4: Infrastructure Layer - Schedule file and today's date

**Red:** `tests/release_catalog_tests.rs`, `tests/calendar_tests.rs`

```text
majors_past_their_end_date_are_end_of_support, the_end_date_itself_is_still_supported,
reports_an_unreadable_release_schedule, formats_a_leap_day_and_the_day_after_it,
today_has_the_date_format
```

**Green:** `src/calendar.rs`, `src/release_catalog.rs`

**Refactoring:** None.

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

1. In a sample folder containing an end-of-support line (`18.20.4`), run without options and check that the upgrade candidate appears.
2. Run with `--write` only and check that the file is unchanged.
3. Run with `--write --upgrade-major` and check that the file changes to the patch version of the target line.

## Considerations

### Reuse of Existing Code

Added the major upgrade judgment to the judgment, report, and apply flow of [Version file scan and fix](../version-file-scan-and-fix/plan.md).

### Compatibility

`--upgrade-major` is shared by this feature and [Aligning version mismatches within a project](../project-version-alignment/spec.md). The major upgrade in both features is enabled by the same option.
Dates are based on UTC, so the result is the same in other time zones.
