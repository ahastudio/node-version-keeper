# Progress Log

> **Update this whenever you complete a phase or run into a problem.**

## Session 2026-10-22

### Phase 1: Requirements & Discovery ✅

**Work done**:

1. Received "bump the major version too if needed" from the request.
2. Decided the target line is the supported LTS line immediately above the current line.
3. Decided to write the major upgrade only when `--write` and `--upgrade-major` are given together.
4. The spec in this document set was written after implementation, based on the decisions in this conversation and the code.

**Files created/modified**:

- `specs/end-of-support-major-upgrade/spec.md` (newly created)

### Phase 2: Planning & Structure ✅

**Work done**:

1. Fetched the schedule file and checked its actual format.
2. Decided the needed changes: reading the schedule, picking the target line, judgment, option, and report section.
3. `plan.md` was written up from the code after implementation.

**Files created/modified**:

- `specs/end-of-support-major-upgrade/plan.md` (newly created)

### Phase 3: Implementation ✅

**Work done**:

1. Read the schedule file to build the end-of-support majors and supported LTS majors.
2. Implemented the end-of-support judgment and target line selection.
3. Implemented the `--upgrade-major` option, `Verdict::MajorUpgrade`, and writing by run mode.
4. Created a module that computes today's date (UTC).
5. This code was implemented together with [Version file scan and fix](../version-file-scan-and-fix/progress.md).

**Files created/modified**:

- `src/calendar.rs` (newly created)
- `src/release_catalog.rs` (newly created, including schedule reading)
- `src/judgement.rs`, `src/verdict.rs`, `src/command_line.rs`, `src/report.rs`, `src/apply.rs` (newly created, including this feature's judgment and option)

### Phase 4: Testing ✅

**Work done**:

1. Tested the end-of-support judgment, end-date boundary, target line selection, writing by mode, and option error.
2. Ran a sample folder against the real official list and confirmed `18.20.4` is upgraded to the patch version of line 22.

**Files created/modified**:

- `tests/calendar_tests.rs` (newly created)
- `tests/release_catalog_tests.rs`, `tests/judgement_tests.rs`, `tests/command_line_tests.rs`, `tests/command_tests.rs`, `tests/report_tests.rs` (newly created)

## Test Results

| Test                                                                      | Input                         | Expected                          | Actual                          | Status |
| ------------------------------------------------------------------------- | ----------------------------- | --------------------------------- | ------------------------------- | ------ |
| `majors_past_their_end_date_are_end_of_support`                           | Schedule file and today's date         | 18, 20, 23 have ended                 | Passed                            | ✅     |
| `the_end_date_itself_is_still_supported`                                  | The end date and the next day         | Supported on the day, ended the next day    | Passed                            | ✅     |
| `upgrade_target_is_the_nearest_supported_lts_above`                       | 18, 23                        | Patch version of 22, patch version of 24    | Passed                            | ✅     |
| `reports_an_end_of_support_line_when_no_supported_lts_is_above`           | Schedule with no supported LTS above     | Report of no line to upgrade to           | Passed                            | ✅     |
| `write_alone_applies_only_same_major_fixes`                               | `--write`                     | End-of-support file unchanged           | Passed                            | ✅     |
| `write_with_upgrade_major_applies_everything_it_can`                      | `--write --upgrade-major`     | End-of-support file changed             | Passed                            | ✅     |
| `upgrading_majors_requires_write`                                         | `--upgrade-major` only           | Error                              | Passed                            | ✅     |
| Sample folder run (real official list)                                           | `18.20.4` in `legacy/.nvmrc`   | Upgraded to patch version of line 22    | `22.23.4`                       | ✅     |

## Error Log

| Timestamp  | Error                                | Attempt | Resolution                       |
| ---------- | ------------------------------------ | ------- | -------------------------------- |
| None       | No errors to record for this feature | -       | -                                   |

## 5-Question Reboot Check

When resuming work, use these questions to recover context:

| Question               | Answer                                                                    |
| ---------------------- | ------------------------------------------------------------------------- |
| 1. Which phase is it now? | Phase 4 complete |
| 2. What is next?     | No work remains in this document |
| 3. What is the goal?             | Upgrade end-of-support lines to a patch version of a supported LTS line           |
| 4. What has been learned so far?   | See findings.md                                                          |
| 5. What has been completed?      | See above                                                              |
