# Feature Specification: Aligning Version Mismatches Within a Project

## Overview

If files in the same project name different Node.js majors, align them to the highest major in that project.
Changing the lower values is a major upgrade, so files are fixed only when the user gives both `--write` and `--upgrade-major`.

## User Scenarios & Testing (mandatory)

### User Story 1: Find mismatches within a project

- As: a developer who runs many projects
- I: can see at a glance when files in the same project name different majors
- So: I don't miss projects that use a different Node.js in each environment

#### Acceptance Scenarios

Scenario 1: **Show the value to align to the highest major**

- Given: in the same folder, `.nvmrc` has `24.11.0`, `Dockerfile` has `FROM node:22.4.0-alpine`, and `volta.node` in `package.json` has `20.10.0`
- When: it runs with no options
- Then: `.nvmrc` appears under "To fix" with `24.22.1`, `Dockerfile` appears under "Version mismatch" with `24.22.1`, and `volta.node`, being an end-of-support line, appears under "End of support" with `24.22.1`; no files change

Scenario 2: **Do not compare with other projects**

- Given: two projects in different folders name `24.22.1` and `22.4.0` respectively
- When: it runs
- Then: the two projects are not aligned to each other, and `22.4.0` is only a same-major fixed-version upgrade

Scenario 3: **A workflow belongs to the project in the folder that holds `.github`**

- Given: `app/.nvmrc` is `24.22.1` and `node-version` in `app/.github/workflows/ci.yml` is `22.4.0`
- When: it runs
- Then: the workflow's value appears as a value to align, `24.22.1`

### User Story 2: Align mismatches in one go

- As: a developer who runs many projects
- I: can explicitly opt into a major upgrade and align all values in a project to the same major
- So: I don't have to fix files one by one

#### Acceptance Scenarios

Scenario 1: **Align with `--write --upgrade-major`**

- Given: `.nvmrc`, `Dockerfile`, and `package.json` name different majors
- When: it runs with `--write --upgrade-major`
- Then: all three files become the fixed version of the highest major

Scenario 2: **Align values in a workflow array too**

- Given: a workflow has `node-version: [18, 22.4.0]` and the highest major in the same project is 24
- When: it runs with `--write --upgrade-major`
- Then: it becomes `node-version: [24, 24.22.1]`

Scenario 3: **With `--write` alone, do not align**

- Given: a project has a mismatch
- When: it runs with `--write` alone
- Then: `--upgrade-major required` is printed on the mismatch line and no files change

### User Story 3: Leave values that can't be compared untouched

- As: a developer who runs many projects
- I: can trust that the tool won't touch values such as ranges or aliases
- So: I avoid changes that alter meaning

#### Acceptance Scenarios

Scenario 1: **Ranges, aliases, variables, and unreadable files are not compared**

- Given: `.nvmrc` has `lts/*`, `package.json` in the same folder is broken JSON, and `Dockerfile` has `22.4.0`
- When: it runs
- Then: `lts/*` and `package.json` are each reported under "Left alone", and `Dockerfile` is only a same-major fixed-version upgrade

## Functional Requirements (mandatory)

- FR-1: MUST treat files in the same folder as one project. Workflow files belong to the project in the folder that holds `.github`.
- FR-2: MUST treat a project as mismatched if it has different majors, judged by the majors after same-major fixes and end-of-support upgrades are determined.
- FR-3: MUST align to the highest major in that project. The new value for a lower one is the fixed version of the highest major.
- FR-4: MUST compare only values that state an exact version or just a major. Ranges, aliases, variables, and unreadable files are not compared.
- FR-5: MUST write mismatch alignment to files only when `--write` and `--upgrade-major` are given together.
- FR-6: MUST preserve the original value's shape (the `v` prefix, exact version or major only).
- FR-7: MUST show mismatch alignment results in the "Version mismatch" section. Values already slated for upgrade due to end of support stay in the "End of support" section.

## Constraints (mandatory)

- CON-1: MUST NOT compare with projects in other folders.
- CON-2: MUST NOT align if the highest major has no fixed version.
- CON-3: MUST leave values already at the highest major as they are.
- CON-4: MUST use only the highest major as the alignment target. No priority among reference files.

## Success Criteria (mandatory)

- SC-1: For a sample project naming different majors, `--write --upgrade-major` makes all values the fixed version of the highest major.
- SC-2: For the same sample, `--write` alone does not apply mismatch alignment.
- SC-3: Projects in other folders do not affect each other.
- SC-4: Running again after alignment reports no mismatch.
