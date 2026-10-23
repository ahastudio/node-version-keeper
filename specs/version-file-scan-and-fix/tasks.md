# Tasks: Version File Scan and Fix

## Goal

Build `node-version-keeper`: given one folder, it finds the Node.js version files in every project under it, compares them with the fixed versions from the official release list, and fixes them to the fixed version of the same major only when `--write` is given.

## Current Phase

✅ Phase 4: Testing

## Phases

### Phase 1: Requirements & Discovery ✅

- [x] Define requirements
- [x] Analyze existing code (not applicable: new project; similar tools were researched by search)
- [x] Write the spec document (spec.md)

### Phase 2: Planning & Structure ✅

- [x] Write the implementation plan (plan.md)
- [x] Detailed analysis of existing code (not applicable: new project)
- [x] Check that related classes/methods exist (not applicable: new project)

### Phase 3: Implementation ✅

- [x] Implement UI Layer — plan.md Step 1
  - [x] Command-line option parsing (`--write`, `--config`, `--exclude`, `--help`)
  - [x] Per-section report and summary line
  - [x] Exit codes
- [x] Implement Application Layer — plan.md Step 2
  - [x] Per-file analysis
  - [x] Writing the values to fix
- [x] Implement Domain Layer — plan.md Step 3
  - [x] Version parsing and value shape classification
  - [x] Same-major fix judgement
  - [x] Computing fixed versions from the official release list
- [x] Implement Infrastructure Layer — plan.md Step 4
  - [x] Detectors for the five kinds of files
  - [x] Folder traversal and excluded folders
  - [x] Reading the config file
  - [x] Downloading the official release list
- [x] Per-step compile verification

### Phase 4: Testing ✅

- [x] Verify the full build
- [x] Verify all tests pass
- [x] Manual testing (where possible)

## Notes

- Update the Phase status as you go: ⏸️ Pending → 🔄 In progress → ✅ Done
- Record decisions in Technical Decisions in findings.md.
- Record errors in Issues Encountered in findings.md.
