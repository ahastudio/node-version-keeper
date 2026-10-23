# Findings & Decisions

> **Update this file immediately whenever there is a technical finding or an important decision.**

## Requirements

- Show values pinned to an end-of-support line as candidates to upgrade to a patch version of a supported LTS line.
- The target line is the supported LTS line immediately above the current line.
- The major upgrade is written to files only when `--write` and `--upgrade-major` are given together.

## Research Findings

### Codebase Structure

- Each entry in the schedule file has `start`, `lts`, `maintenance`, and `end` per major. An entry without `lts` is a non-LTS line.
- The schedule file also has keys containing a decimal point, such as `v0.10`, so only keys made up of digits are accepted when reading major keys.

### Existing Patterns

- End of support and LTS start are judged by lexicographic comparison of date strings (`YYYY-MM-DD`). The dates have the same length, so lexicographic order matches chronological order.
- Running against the real official list showed that line 20 has already ended, so the upgrade candidate for `18.20.4` was the patch version of 22, not 20.

## Resources

### Documentation

- [Node.js release schedule](https://raw.githubusercontent.com/nodejs/Release/main/schedule.json)

### Code References

- End-of-support and supported-LTS calculation: `src/release_catalog.rs`
- Judgment of end-of-support lines: `src/judgement.rs`
- Decision that splits the value to write by mode: `src/verdict.rs`

### API Endpoints

- GET `https://raw.githubusercontent.com/nodejs/Release/main/schedule.json`

## Technical Decisions

| Decision | Rationale |
| -------- | --------- |
| The target line is the supported LTS line immediately above the current line | It skips the fewest majors, so the risk of breaking the app is smallest. The user chose the nearest supported LTS over the latest active LTS |
| The major upgrade is written only when `--write` and `--upgrade-major` are given together | A major upgrade can break the app. The preview only shows the upgrade candidate |
| `--upgrade-major` alone ends with an error | The preview already shows candidates, so this option alone has nothing to do |
| A line is treated as supported through its end date | It is judged end-of-support from the day after the end date |
| Only lines whose LTS start date is on or before today are target lines | A line whose LTS has not started yet is not a supported LTS |
| LTS lines with no patch version are skipped | The value to upgrade to cannot be determined |
| Dates are computed in UTC inside the tool | It gives the same result on the same day regardless of time zone, and avoids an external package for date calculation |

## Issues Encountered

There were no issues to record for this feature.
Method bodies exceeding 5 lines and missing line coverage are recorded in [Version file scan and fix](../version-file-scan-and-fix/findings.md).

## Learnings

### Test both boundary days of the end-date judgment (2026-10-22)

The judgment on the end date and on the day after it differ.
We added a test that reads the same schedule file twice, changing only today's date.
