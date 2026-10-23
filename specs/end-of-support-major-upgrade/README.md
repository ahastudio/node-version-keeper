# Major Upgrade for End-of-Support Lines

## Background

Node.js lines that have reached end of support no longer receive security releases.
Bumping to a patch version within the same major does not fix this, so the major must be upgraded to a supported line.
The Node.js security release announcements also state that end-of-support versions are always affected by security releases.

## Goal

Show values pinned to an end-of-support line as candidates to upgrade to a patch version of a supported LTS line, and modify files only when the major upgrade is explicitly requested.

## How it works

### Preview

- A major whose end date is before today (UTC) in the schedule file is treated as an end-of-support line. The line is still supported through its end date.
- Values that pin an exact version or only a major of an end-of-support line are shown in the "End of support" section. This section appears at the very top of the report.
- The target line is the supported LTS line immediately above the current line. Its patch version is listed as the new value.
- If there is no target line, nothing is changed and the report says "no line to upgrade to".

### Fix

- The major upgrade is written to files only when `--write` and `--upgrade-major` are given together.
- With `--write` alone, only same-major fixes are applied, and the major upgrade line says `--upgrade-major required`.
- With `--upgrade-major` alone, the run ends with an error.

## Related Documents

- [spec.md](spec.md): Requirements and detailed spec
- [plan.md](plan.md): Technical implementation plan
- [tasks.md](tasks.md): Task plan and tracking
- [findings.md](findings.md): Technical findings and decisions
- [progress.md](progress.md): Per-session work log
- [Version file scan and fix](../version-file-scan-and-fix/README.md)
- [Aligning version mismatches within a project](../project-version-alignment/README.md)
