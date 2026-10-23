# Version File Scan and Fix

## Background

With many projects, it is easy to miss bumping versions for Node.js security releases.
Node.js versions are scattered across `.nvmrc`, `.node-version`, `.tool-versions`, `package.json`, `Dockerfile`, and workflow files, so they have to be hunted down project by project.

## Goal

Given one folder, find the files that specify a Node.js version in every project under it, compare them with the fixed versions from the official release list, and report the result.
On request, fix values that can be raised to the fixed version of the same major.

## How it works

### Preview

- Run it as `node-version-keeper <folder>`. Without options, it does not change any file.
- It walks the folder and finds Node.js version values in five kinds of files. It skips `node_modules`, `.git`, and the folders listed in the config file.
- On every run, it downloads the fixed version for each major from the official release list and compares against it.
- The result is shown as per-section lists and a summary line. Each line gives the file path, line number, current value, and the new value or the reason it was not fixed.
- The exit code is 1 if unresolved values remain, 0 otherwise.

### Fix

- With `--write`, an exact version lower than the fixed version is changed to the fixed version of the same major.
- The `v` prefix and suffixes such as `-alpine` in Dockerfile tags are left as they are.
- Values that give only a major, ranges, aliases, and variables are not changed and are reported with a reason.
- Major upgrades for end-of-support lines and aligning version mismatches within a project are separate features, fixed with `--upgrade-major`.

## Related Documents

- [spec.md](spec.md): Requirements and detailed spec
- [plan.md](plan.md): Technical implementation plan
- [tasks.md](tasks.md): Task plan and tracking
- [findings.md](findings.md): Technical findings and decisions
- [progress.md](progress.md): Per-session work log
- [Major upgrade for end-of-support lines](../end-of-support-major-upgrade/README.md)
- [Version alignment within a project](../project-version-alignment/README.md)
