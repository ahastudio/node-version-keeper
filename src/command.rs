use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use crate::alignment::align_projects;
use crate::analysis::{FileAnalysis, analyze_file};
use crate::apply::apply_all;
use crate::calendar::today_utc;
use crate::command_line::{Command, Options, parse_arguments};
use crate::config::{excluded_names, load_config};
use crate::error::KeeperError;
use crate::file_scanner::collect_candidate_files;
use crate::http_source::{HttpReleaseSource, ReleaseSource};
use crate::release_catalog::{ReleaseCatalog, load_catalog};
use crate::report::{exit_code, render_report};

const HELP_TEXT: &str = "\
Usage: node-version-keeper <folder> [options]

Finds the Node.js versions written in project files under the folder and compares them with the patch versions in the official release list.
Without options, it only shows the result and changes no files.

Options:
  --write            Fix values that can be raised to a patch version of the same major
  --upgrade-major    Also fix major upgrades for end-of-support lines and version mismatches. Use with --write
  --config <path>    Excluded-folder config file. Defaults to the built-in config
  --exclude <name>   Folder name to exclude in this run. Can be given multiple times
  --help             Show this help
";

// Runs in the real environment (command-line arguments, official release list, today's date, standard output)
pub fn run_from_environment() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let outcome = run(&arguments, &HttpReleaseSource, &today_utc(), &mut std::io::stdout());
    exit_code_of(outcome)
}

// Returns the exit code. 0 means nothing to resolve, 1 means unresolved values remain
pub fn run(
    arguments: &[String],
    source: &dyn ReleaseSource,
    today: &str,
    output: &mut dyn Write,
) -> Result<i32, KeeperError> {
    match parse_arguments(arguments)? {
        Command::Help => print_help(output),
        Command::Run(options) => run_options(&options, source, today, output),
    }
}

fn exit_code_of(outcome: Result<i32, KeeperError>) -> ExitCode {
    match outcome {
        Ok(code) => ExitCode::from(code as u8),
        Err(error) => report_error(&error),
    }
}

fn report_error(error: &KeeperError) -> ExitCode {
    eprintln!("Error: {error}");
    ExitCode::from(2)
}

fn print_help(output: &mut dyn Write) -> Result<i32, KeeperError> {
    write_output(output, HELP_TEXT)?;
    Ok(0)
}

fn run_options(
    options: &Options,
    source: &dyn ReleaseSource,
    today: &str,
    output: &mut dyn Write,
) -> Result<i32, KeeperError> {
    let catalog = load_catalog(source, today)?;
    let analyses = analyze_folder(options, &catalog)?;
    apply_all(&analyses, options.apply_mode())?;
    write_output(output, &render_report(&analyses, options.apply_mode()))?;
    Ok(exit_code(&analyses, options.apply_mode()))
}

fn analyze_folder(options: &Options, catalog: &ReleaseCatalog) -> Result<Vec<FileAnalysis>, KeeperError> {
    let files = scan_files(options)?;
    let mut analyses = files.iter().map(|path| analyze_file(path, catalog)).collect::<Result<Vec<_>, _>>()?;
    align_projects(&mut analyses, catalog);
    Ok(analyses)
}

fn scan_files(options: &Options) -> Result<Vec<PathBuf>, KeeperError> {
    let config = load_config(options.config_path.as_deref())?;
    let excluded = excluded_names(&config, &options.exclude);
    collect_candidate_files(&options.folder, &excluded)
}

fn write_output(output: &mut dyn Write, text: &str) -> Result<(), KeeperError> {
    output.write_all(text.as_bytes()).map_err(|error| KeeperError::new(format!("output failed: {error}")))
}
