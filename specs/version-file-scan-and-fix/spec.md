# Feature Specification: Version File Scan and Fix

## Overview

Given one folder, this tool finds the files that specify a Node.js version in every project under it, compares them with the fixed versions from the official release list and reports the result, and on request fixes them to the fixed version of the same major.
Run without options, it shows only the result and does not change any file.

## User Scenarios & Testing (mandatory)

### User Story 1: Check all projects with one folder

- As: a developer who runs many projects
- I: can check the files that specify a Node.js version in one pass by giving a single folder
- So: that I do not miss security releases

#### Acceptance Scenarios

Scenario 1: **Running without options does not change files**

- Given: a folder containing several projects, where some files specify a version lower than the fixed version
- When: it is run without options
- Then: the values to fix appear as per-section lists and a summary line, and no file is changed

Scenario 2: **Excluded folders are not scanned**

- Given: version files exist under `node_modules` and `dist`
- When: it is run
- Then: files under those two folders do not appear in the result

### User Story 2: Fix to the same major's fixed version in one go

- As: a developer who runs many projects
- I: can fix every value that can be raised to the fixed version of the same major in one go
- So: that security releases are applied without manual work

#### Acceptance Scenarios

Scenario 1: **Raise a lower version to the fixed version**

- Given: `.nvmrc` contains `24.11.0` and the fixed version of the 24 line is `24.22.1`
- When: it is run with `--write`
- Then: the file becomes `24.22.1`, and the exit code is 0 if no unresolved values remain

Scenario 2: **Keep the shape of the value**

- Given: a file containing `v24.11.0` and a Dockerfile containing `FROM node:24.11.0-alpine`
- When: it is run with `--write`
- Then: they become `v24.22.1` and `FROM node:24.22.1-alpine`, keeping the prefix and the tag suffix

### User Story 3: See values that were not fixed, with reasons

- As: a developer who runs many projects
- I: can see the values the tool did not fix, with reasons
- So: that I do not miss values I have to judge myself

#### Acceptance Scenarios

Scenario 1: **Report values that were not fixed**

- Given: a value giving only `24` on a supported line, and a `>=22` range
- When: it is run
- Then: both values appear in the "Left alone" section with reasons, and the files are not changed

### User Story 4: Set excluded folders in a config file

- As: a developer who runs many projects
- I: can set the folders to exclude with a config file and options
- So: that files in build output folders are left out of the check

#### Acceptance Scenarios

Scenario 1: **The default config file excludes build output folders**

- Given: version files exist under `dist`
- When: it is run without a config file
- Then: files under `dist` do not appear in the result

Scenario 2: **Another config file replaces the default config file**

- Given: a config file that does not list `dist` is passed with `--config`
- When: it is run
- Then: files under `dist` also appear in the result, while files under `node_modules` and `.git` still do not

## Functional Requirements (mandatory)

- FR-1: MUST find every Node.js version specified in `.nvmrc`, `.node-version`, `.tool-versions`, `package.json` (`engines.node`, `volta.node`), `Dockerfile` kinds, and workflows (`node-version`) under the given folder.
- FR-2: MUST determine the fixed version from the official Node.js release list as the most recent security release of that major. A major with no security release is treated as having nothing to fix.
- FR-3: MUST judge an exact version lower than the fixed version as a value to raise to the fixed version of the same major. If it is at or above the fixed version, do nothing and do not include it in the report.
- FR-4: MUST NOT change files when run without options. Files are fixed only when `--write` is given.
- FR-5: MUST keep the shape of the original value when fixing (the `v` prefix, whether it was an exact version or only a major, and the suffix of a Dockerfile tag).
- FR-6: MUST NOT fix, and MUST report with a reason, values that give only a major on a supported line, ranges, aliases, variables, and `package.json` files that could not be read.
- FR-7: MUST always exclude `node_modules` and `.git`.
- FR-8: MUST exclude the folders listed in `exclude` of the config file `node-version-keeper-config.json`. The tool ships this config file from the start, containing `dist`, `build`, `.next`, `coverage`, and `vendor`.
- FR-9: SHOULD accept another config file with `--config <path>`. When given, only that file is read instead of the default config file.
- FR-10: SHOULD allow excluding more folders for that run only with `--exclude <name>`. It can be given multiple times.
- FR-11: MUST show the result as per-section lists and a final summary line. Each line gives the file path, line number, current value, and the new value or the reason it was not fixed.
- FR-12: MUST use exit code 0 if there is nothing to resolve, 1 if unresolved values remain, and 2 if it could not run.
- FR-13: SHOULD show usage with `--help`.

## Constraints (mandatory)

- CON-1: MUST end with exit code 2 without changing any file if the official release list cannot be downloaded.
- CON-2: MUST determine the fixed version automatically only. No version table is kept inside the tool, and no option to specify it directly is provided.
- CON-3: MUST NOT exclude hidden folders wholesale, because `.github/workflows` has to be read.
- CON-4: MUST NOT follow symbolic links.
- CON-5: MUST change only the place in a file where the version value is, leaving the rest of the content and its shape as is.

## Success Criteria (mandatory)

- SC-1: Running without options on a sample folder containing lower versions changes no file.
- SC-2: With `--write` on the same sample folder, `24.11.0` becomes `24.22.1` and the rest of the same file is unchanged.
- SC-3: Files under `node_modules` and `dist` do not appear in the result with the default config.
- SC-4: Even with `--write`, no file is changed when the official release list cannot be downloaded.
- SC-5: `mise run cover` passes with 100% line coverage across `src/`.
