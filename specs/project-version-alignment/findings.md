# Findings & Decisions

> **Update this file immediately whenever there is a technical finding or an important decision.**

## Requirements

- Align values that name different majors within the same project to the highest major.
- Changing the lower values is a major upgrade, so write it only with `--write --upgrade-major`.
- Leave values that are not compared untouched.

## Research Findings

### Codebase structure

- `Observation` holds both the value's classification (`classification`) and the verdict (`verdict`), so the original value's shape is still known when aligning.
- If the verdict is `Fix` or `MajorUpgrade`, the value after the verdict is `replacement`; otherwise it is the original value.

### Existing patterns

- Running a sample against the real official list turned a workflow's `node-version: [18, 22.4.0]` into `[24, 24.22.1]`. 18 is end of support, so it goes up through 22 to 24.
- Running again after alignment leaves the major-only `24` under "Left alone".

## Resources

### Documents

- Verdicts and major upgrade this feature relies on: [Finding and fixing version files](../version-file-scan-and-fix/spec.md), [Major upgrade for end-of-support lines](../end-of-support-major-upgrade/spec.md)

### Code references

- Per-project alignment: `src/alignment.rs`
- Determining the project folder: `src/detection.rs`

### API endpoints

- None. This feature calls no new external address.

## Technical Decisions

| Decision | Rationale |
| -------- | --------- |
| Treat files in the same folder as one project | A project's version files are generally in the same folder. Other folders are not compared |
| A workflow belongs to the project in the folder that holds `.github` | Workflows sit under `.github/workflows`, so grouping by folder alone would put them in a different folder from the project root's files |
| Align to the highest major | The user chose the highest major from among highest major, reference-file priority, and report only |
| Compare the major after individual verdicts are applied | End-of-support upgrades must be reflected first. For example, 18 goes up to 22 and is then aligned to 24 |
| Do not compare ranges, aliases, variables, or unreadable files | Their major can't be known |
| Do not align if the highest major has no fixed version | The new value can't be determined |
| Keep end of support as the cause for values already slated for upgrade because of it | They keep showing in the "End of support" section |
| Write alignment as a major upgrade | Raising lower values needs `--write --upgrade-major` |

## Issues Encountered

### 1. Wrong expected count in the re-run summary after alignment

**Problem**:

`running_again_after_the_upgrade_leaves_only_the_reports` failed expecting 2 items under "Left alone". The actual count was 3.

**Cause**:

A workflow's `node-version: [18, 22.4.0]` became `[24, 24.22.1]`, adding one major-only value, `24`.

**Resolution**:

Changed the expected count to 3.

**Result**:

The test passed.

## Learnings

### Alignment can add major-only values (2026-10-22)

If the lower value was in major-only form, the aligned result is also major-only.
On the next run, such values remain under "Left alone".
