# Feature Specification: Major Upgrade for End-of-Support Lines

## Overview

Show values pinned to an end-of-support Node.js line as candidates to upgrade to a patch version of a supported LTS line, and modify files only when the user explicitly requests the major upgrade.
Fixes within the same major are covered by [Version file scan and fix](../version-file-scan-and-fix/spec.md).

## User Scenarios & Testing (mandatory)

### User Story 1: See end-of-support lines together with upgrade candidates

- As: a developer who maintains many projects
- I: can see, at once, the values pinned to end-of-support Node.js lines and their upgrade candidates
- So: that I do not miss projects using lines that can no longer receive security releases

#### Acceptance Scenarios

Scenario 1: **Show the upgrade candidate for an exact version**

- Given: `.nvmrc` contains `18.20.4`, line 18 has reached end of support, and the supported LTS line immediately above is 22
- When: run without options
- Then: the "End of support" section shows `18.20.4` and the patch version of line 22 along with `--write and --upgrade-major to fix`, and the file is not changed

Scenario 2: **Show an upgrade candidate for a major-only value too**

- Given: the file contains only `18`
- When: run without options
- Then: the new value is shown as `22`

### User Story 2: Modify only when the major upgrade is explicit

- As: a developer who maintains many projects
- I: can make files change only when I explicitly request a major upgrade
- So: that I do not accidentally apply a change that could break the app

#### Acceptance Scenarios

Scenario 1: **With `--write` alone, the major upgrade is not applied**

- Given: a file containing `18.20.4`
- When: run with `--write` only
- Then: the file is unchanged and the line says `--upgrade-major required`

Scenario 2: **With `--write` and `--upgrade-major` together, it is fixed**

- Given: a file containing `18.20.4`
- When: run with `--write --upgrade-major`
- Then: the file is changed to the patch version of line 22

Scenario 3: **With `--upgrade-major` alone, the run ends with an error**

- Given: a folder is provided
- When: run with `--upgrade-major` only
- Then: it exits with code 2 and an error saying `--upgrade-major must be used with --write`

### User Story 3: Tell me when there is no line to upgrade to

- As: a developer who maintains many projects
- I: can tell apart the case where there is no supported line to upgrade to
- So: that I do not miss values I must decide on myself

#### Acceptance Scenarios

Scenario 1: **With no target line, only report**

- Given: there is a value on an end-of-support line and no supported LTS line above it
- When: run
- Then: the "End of support" section shows the reason `no line to upgrade to` and the file is not modified

## Functional Requirements (mandatory)

- FR-1: MUST treat a major whose end date in the schedule file is before today (UTC) as an end-of-support line.
- FR-2: MUST judge values that pin an exact version or only a major of an end-of-support line as upgrade candidates.
- FR-3: MUST pick the target line as the supported LTS line immediately above the current line. A supported LTS is a major whose LTS start date is on or before today and whose end date has not passed.
- FR-4: MUST make the new value the patch version of the target line. Preserve the shape of the original value (the `v` prefix, exact version or major only).
- FR-5: MUST, if no target line is found, leave files unmodified and report "no line to upgrade to".
- FR-6: MUST write the major upgrade to files only when `--write` and `--upgrade-major` are given together.
- FR-7: MUST end with an error if `--upgrade-major` is given without `--write`.
- FR-8: MUST show end-of-support results in the "End of support" section, and place this section at the very top of the report.

## Constraints (mandatory)

- CON-1: MUST treat a line as supported through its end date.
- CON-2: MUST NOT use a line whose LTS start date is after today as a target line.
- CON-3: MUST skip an LTS line with no patch version rather than use it as a target line.
- CON-4: MUST NOT upgrade ranges, aliases, or variables, since the line cannot be determined; only report them.
- CON-5: MUST fetch the schedule and patch versions from the official source at run time. If they cannot be fetched, do not modify files.

## Success Criteria (mandatory)

- SC-1: With the real official release list, `18.20.4` becomes an upgrade candidate to the patch version of line 22.
- SC-2: With `--write` alone, files on end-of-support lines are not changed.
- SC-3: On the end date the line is judged supported, and from the next day it is judged end-of-support.
- SC-4: Values with no target line are not modified and are left in the report.
