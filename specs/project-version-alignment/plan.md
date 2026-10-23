# Implementation Plan: Aligning Version Mismatches Within a Project

## Summary

After all per-file verdicts are done, group values by project folder and align lower values to the highest of the post-verdict majors.
Aligned values are major-upgrade verdicts, so when they are written follows the same rule as major upgrades.

## Requirements

1. Group files in the same folder into one project, and make workflows belong to the folder that holds `.github`. (FR-1)
2. Compare the major after individual verdicts are applied. (FR-2)
3. Align lower values to the fixed version of the highest major. (FR-3, FR-6)
4. Do not compare ranges, aliases, variables, or unreadable files. (FR-4)
5. Write only with `--write --upgrade-major`. (FR-5)
6. Show in the "Version mismatch" section, and leave end-of-support upgrades in their original section. (FR-7)

## Critical Files

### New Files

- `src/alignment.rs`
- `tests/alignment_tests.rs`

### Modified Files

- `src/detection.rs`: `project_folder`, which determines the project folder
- `src/command.rs`: `analyze_folder` calls `align_projects` after analysis
- `src/report.rs`: the "Version mismatch" section
- `src/verdict.rs`: `UpgradeCause::Mismatch`
- `src/version_text.rs`: `shape_of`, `major_of`, `render`

### Reference Files

- `src/observation.rs`: holds both the value's classification (`classification`) and the verdict (`verdict`)
- `src/release_catalog.rs`: fixed version per major

## Architecture

### User Flow

```text
User → [node-version-keeper <folder> --write --upgrade-major] → per-file verdicts
                                                                    ↓
                                                  group by same project
                                                                    ↓
                                            align lower values to the highest major
                                                                    ↓
                                                  report and file writes
```

### Event Flow

```text
1. command::analyze_folder: judge each file with analyze_file
       ↓
2. alignment::align_projects: build per-project groups with detection::project_folder
       ↓
3. alignment::align_group: find the highest post-verdict major in the group
       ↓
4. release_catalog: find that major's fixed version (if none, do not align)
       ↓
5. alignment::align_one: turn the lower value's verdict into a major upgrade
```

### Domain Model

```text
Observation (classification, verdict)
└── verdict
    ├── Fix(replacement)                          ← post-verdict major is replacement's major
    └── MajorUpgrade { replacement, cause }
        └── cause: EndOfSupport or Mismatch        ← when aligning, an existing EndOfSupport is kept
```

## Implementation Steps

### Step 1: UI Layer - Version mismatch section of the report

**Red:** `tests/command_tests.rs`

```text
preview_lists_everything_and_changes_nothing,
write_with_upgrade_major_applies_everything_it_can,
running_again_after_the_upgrade_leaves_only_the_reports
```

**Green:** `src/report.rs`

**Refactoring:** Split `summary` into `counts_text` and `count_phrase`.

### Step 2: Application Layer - Per-project grouping and alignment

**Red:** `tests/alignment_tests.rs`

```text
aligns_every_lower_major_in_a_project_to_the_highest_major,
does_not_align_across_projects,
a_workflow_belongs_to_the_project_holding_dot_github
```

**Green:** `src/alignment.rs`, `src/command.rs`

**Refactoring:** Split the repeated body of `group_by_project` into `add_members`.

### Step 3: Domain Layer - Alignment verdict

**Red:** `tests/alignment_tests.rs`

```text
aligns_a_value_that_is_already_current_and_keeps_the_prefix,
aligns_a_major_only_value_keeping_its_shape,
does_not_align_when_the_highest_major_has_no_fixed_version,
ignores_ranges_aliases_and_unreadable_files_when_comparing_majors
```

**Green:** `src/alignment.rs`, `src/version_text.rs`, `src/verdict.rs`

**Refactoring:** Cut the `match` in `final_major` to 5 lines or fewer.

### Step 4: Infrastructure Layer - Determining the project folder

**Red:** `tests/detection_tests.rs`

```text
project_folder_is_the_folder_of_the_file,
project_folder_of_a_workflow_is_the_folder_holding_dot_github,
project_folder_of_a_root_path_is_itself
```

**Green:** `src/detection.rs`

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

1. In a sample folder with `.nvmrc`, `Dockerfile`, `package.json`, and a workflow naming different majors, run with no options and check the "Version mismatch" section.
2. Run with `--write --upgrade-major` and check that every value became the fixed version of the highest major.
3. Run again and check that no mismatch appears.

## Considerations

### Reusing existing code

Use `Observation` and `Verdict` from [Finding and fixing version files](../version-file-scan-and-fix/plan.md) and `MajorUpgrade` from [Major upgrade for end-of-support lines](../end-of-support-major-upgrade/plan.md) as they are.
Alignment is post-processing that rewrites already-made verdicts based on the highest major.

### Compatibility

After alignment, a major-only value (`24`) may appear. On the next run, that value remains under "Left alone".
Files not in the same folder are treated as different projects even within the same repository.
