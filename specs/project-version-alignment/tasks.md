# Tasks: Aligning Version Mismatches Within a Project

## Goal

If a project's Node.js version values name different majors, align them to the highest major, and fix files only when `--write` and `--upgrade-major` are given together.

## Current Phase

✅ Phase 4: Testing

## Phases

### Phase 1: Requirements & Discovery ✅

- [x] Define requirements
- [x] Analyze existing code (checked verdicts, major upgrade, and report sections)
- [x] Write the spec document (spec.md)

### Phase 2: Planning & Structure ✅

- [x] Write the implementation plan (plan.md)
- [x] Analyze existing code in detail (confirmed `Observation` holds both the value's classification and the verdict)
- [x] Confirm related classes/methods exist (`analyze_file`, `fixed_version`, `render`)

### Phase 3: Implementation ✅

- [x] Implement UI Layer — plan.md Step 1
  - [x] "Version mismatch" section and summary line
- [x] Implement Application Layer — plan.md Step 2
  - [x] Group values by project
  - [x] Align each group to its highest major
- [x] Implement Domain Layer — plan.md Step 3
  - [x] Compute the major after verdicts are applied
  - [x] Turn lower values into major-upgrade verdicts
- [x] Implement Infrastructure Layer — plan.md Step 4
  - [x] Determine the project folder from a file path
- [x] Compile check at each step

### Phase 4: Testing ✅

- [x] Confirm full build
- [x] Confirm all tests pass
- [x] Manual test (where possible)

## Notes

- Update the Phase status as you go: ⏸️ Waiting → 🔄 In progress → ✅ Done
- Record decisions in Technical Decisions in findings.md.
- Record errors in Issues Encountered in findings.md.
