# Aligning Version Mismatches Within a Project

## Background

A project's `.nvmrc`, `Dockerfile`, `package.json`, and workflows may name different Node.js majors.
There was a request to resolve this mismatch between versions in the same project.

## Goal

Align a project's Node.js version values to the highest major in that project.
Changing the lower values is a major upgrade, so they are fixed only when `--write` and `--upgrade-major` are given together.

## How it works

### Preview

- Files in the same folder are treated as one project. Workflow files belong to the project in the folder that holds `.github`.
- If majors still differ after the same-major fix and end-of-support upgrade verdicts are applied first, it counts as a mismatch.
- The fixed version of the highest major is shown as the new value. This line appears in the "Version mismatch" section.
- A value already slated for upgrade because of end of support stays in the "End of support" section; only its new value is aligned to the highest major.

### Fixing

- Alignment is written to files when `--write --upgrade-major` is given.
- With `--write` alone, nothing is written and `--upgrade-major required` is printed.
- Ranges, aliases, variables, and unreadable files are not compared and are left untouched.

## Related Documents

- [spec.md](spec.md): Requirements and detailed spec
- [plan.md](plan.md): Technical implementation plan
- [tasks.md](tasks.md): Task plan and tracking
- [findings.md](findings.md): Technical findings and decisions
- [progress.md](progress.md): Per-session work log
- [Finding and fixing version files](../version-file-scan-and-fix/README.md)
- [Major upgrade for end-of-support lines](../end-of-support-major-upgrade/README.md)
