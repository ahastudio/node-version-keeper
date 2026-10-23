# node-version-keeper

A tool that finds the files stating a Node.js version in every project under a folder, compares them with the patch versions in the official Node.js release list, and bumps them.

Use it when you have so many projects that security releases are easy to miss.
Without options it changes no files and only shows the result.

## Files it looks at

| File | Where the version is read |
| -- | -- |
| `.nvmrc`, `.node-version` | The first line, skipping comments and blank lines |
| `.tool-versions` | The versions on the `nodejs` or `node` line |
| `package.json` | `engines.node`, `volta.node` |
| `Dockerfile`, `Dockerfile.*`, `*.Dockerfile` | The tag of a `FROM node:<tag>` line, without a trailing part such as `-alpine` |
| `.github/workflows/*.yml`, `*.yaml` | The `node-version` value. A single value, a one-line array such as `[20, 22]`, and a multi-line list are all read |

## Usage

```bash
mise install
mise run build
./target/release/node-version-keeper <folder>
```

| Run | What it does |
| -- | -- |
| `node-version-keeper <folder>` | Shows what it found. Changes no files |
| `node-version-keeper <folder> --write` | Fixes values that can be raised to a patch version of the same major |
| `node-version-keeper <folder> --write --upgrade-major` | Also fixes major upgrades |

| Option | Meaning |
| -- | -- |
| `--write` | Actually modify the files |
| `--upgrade-major` | Also fix major upgrades for end-of-support lines and version mismatches. A major upgrade can break an app, so it is only applied together with `--write` |
| `--config <path>` | Excluded-folder config file. Defaults to the config built into the tool |
| `--exclude <name>` | A folder name to exclude in this run. Can be given multiple times |
| `--help` | Show the help |

Example output:

```text
End of support 1
  ./legacy/.nvmrc:1  18.20.4  → 22.23.4 (major upgrade, --write and --upgrade-major to fix)

Version mismatch 1
  ./app/Dockerfile:1  22.4.0  → 24.22.1 (major upgrade, --write and --upgrade-major to fix)

To fix 1
  ./app/.nvmrc:1  24.11.0  → 24.22.1 (--write to fix)

Left alone 2
  ./app/Dockerfile:2  24  only major 24 is given, left unchanged
  ./app/package.json:3  >=22  not an exact version (range, alias, or variable), left unchanged

Summary: 5 files checked, End of support 1, Version mismatch 1, To fix 1, Left alone 2
```

Exit codes:

| Code | Meaning |
| -- | -- |
| 0 | Nothing is left to resolve. Also 0 when `--write` fixed everything |
| 1 | Some values are unresolved: the preview found something to fix, or a value was only reported and not fixed |
| 2 | The run failed: a folder could not be read, the release list could not be fetched, or an option was invalid |

## Decision rules

Each value found in a file is judged by the rules below.

| Value in the file | Example | What happens |
| -- | -- | -- |
| An exact version lower than the patch version | `24.11.0` | Raised to the patch version of the same major (`--write`) |
| An exact version at or above the patch version | `24.22.1` | Nothing happens and it is not reported |
| An exact version or bare major on an end-of-support line | `18.20.4`, `18` | Raised to the patch version of the nearest supported LTS line above it (`--write --upgrade-major`). Reported only if there is no line to upgrade to |
| A bare major on a supported line | `24`, `v24` | Reported only |
| A range, alias, or variable | `>=22`, `lts/*`, `${NODE_VERSION}` | Reported only |
| An unreadable `package.json` | Broken JSON | Reported only |

- The **patch version** is the most recent security release of that major in the official release list (the entry with `security` set to `true`). A major with no security release is treated as having nothing to fix.
- An **end-of-support line** is a major whose `end` date in the official schedule file is before today (UTC). A line counts as supported through its end date.
- A **supported LTS** is a major whose `lts` start date in the schedule file is before today and whose `end` has not passed.
- A **version mismatch** is when files in the same project still have different majors after the rules above are applied. Everything is aligned to the highest major, and the lower ones are major upgrades, so they are fixed with `--write --upgrade-major`. Ranges, aliases, variables, and unreadable files are not compared.
- The **same project** means files in the same folder. A workflow file belongs to the folder that contains `.github`.
- The original shape of the value is kept. A `v` prefix and whether it was an exact version or a bare major stay as they were. A Dockerfile's trailing part such as `-alpine` is left untouched.

## Data sources

Every run downloads from these two places. The tool contains no version table, so it needs no change when a new security release comes out, but it does need a network connection. If the download fails, it touches no files and exits with code 2.

- Release list: `https://nodejs.org/dist/index.json`
- Support schedule: `https://raw.githubusercontent.com/nodejs/Release/main/schedule.json`

## Excluded folders

`node_modules` and `.git` are always excluded, whatever config you give. Hidden folders as a whole are not excluded, because `.github/workflows` has to be read.

The default config file [node-version-keeper-config.json](node-version-keeper-config.json) is built into the tool and excludes these additional folders:

```json
{
  "exclude": ["dist", "build", ".next", "coverage", "vendor"]
}
```

If you pass another config file with `--config <path>`, it is read instead of the default one. The two files are not merged, so a file you write yourself must list every folder to exclude. Symbolic links are not followed.

## What it does not do

- It does not look at the Node.js version of running processes or containers. It only looks at the values written in files.
- It does not fix ranges (`>=22`) or aliases (`lts/*`). Changing the value would change the allowed range or the meaning.
- It does not read versions passed in through a Dockerfile `ARG` or images of the form `node@sha256:`.
- It does not read a workflow's `node-version-file`. It reads the `.nvmrc` or similar file that points to.
- It does not check that an image tag or package for the raised version actually exists.

## Development

The Rust version and tools are set by [mise.toml](mise.toml).

| Command | What it does |
| -- | -- |
| `mise install` | Install Rust and `cargo-llvm-cov` |
| `mise run build` | Build the executable |
| `mise run test` | Run the tests |
| `mise run cover` | Measure total line coverage and fail if it is below 100% |
