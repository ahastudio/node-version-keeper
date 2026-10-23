# Tasks: Major Upgrade for End-of-Support Lines

## Goal

Show values pinned to an end-of-support Node.js line as candidates to upgrade to a patch version of a supported LTS line, and modify files only when `--write` and `--upgrade-major` are given together.

## Current Phase

✅ Phase 4: Testing

## Phases

### Phase 1: Requirements & Discovery ✅

- [x] Define requirements
- [x] Analyze existing code (reviewed the judgment, report, and apply flow of version file scan and fix)
- [x] Write spec document (spec.md)

### Phase 2: Planning & Structure ✅

- [x] Write implementation plan (plan.md)
- [x] Detailed analysis of existing code (identified where to add the judgment and report section)
- [x] Confirm related classes/methods exist (`judge`, `render_report`, `apply_all`)

### Phase 3: Implementation ✅

- [x] Implement UI Layer — plan.md Step 1
  - [x] `--upgrade-major` option and the error when given without `--write`
  - [x] "End of support" section and apply status text
- [x] Implement Application Layer — plan.md Step 2
  - [x] Write only the values the run mode allows
- [x] Implement Domain Layer — plan.md Step 3
  - [x] Judge values on end-of-support lines as values to upgrade
  - [x] Pick the line immediately above among supported LTS lines
- [x] Implement Infrastructure Layer — plan.md Step 4
  - [x] Read `lts` and `end` from the schedule file
  - [x] Compute today's date (UTC)
- [x] Verify compilation at each step

### Phase 4: Testing ✅

- [x] Confirm full build
- [x] Confirm all tests pass
- [x] Manual testing (where possible)

## Notes

- Update the Phase status as you progress: ⏸️ Pending → 🔄 In progress → ✅ Done
- Record decisions in Technical Decisions in findings.md.
- Record errors in Issues Encountered in findings.md.
