use std::path::PathBuf;

use node_version_keeper::command_line::{Command, Options, parse_arguments};

fn parse(arguments: &[&str]) -> Result<Command, String> {
    let owned: Vec<String> = arguments.iter().map(|argument| argument.to_string()).collect();
    parse_arguments(&owned).map_err(|error| error.to_string())
}

#[test]
fn a_folder_alone_is_a_preview_run() {
    let expected = Options {
        folder: PathBuf::from("projects"),
        write: false,
        upgrade_major: false,
        config_path: None,
        exclude: Vec::new(),
    };
    assert_eq!(parse(&["projects"]), Ok(Command::Run(expected)));
}

#[test]
fn reads_every_option() {
    let arguments =
        ["--write", "--upgrade-major", "--config", "my.json", "projects", "--exclude", "a", "--exclude", "b"];
    let Ok(Command::Run(options)) = parse(&arguments) else { panic!("expected a run command") };
    assert_eq!(options.folder, PathBuf::from("projects"));
    assert!(options.write && options.upgrade_major);
    assert_eq!(options.config_path, Some(PathBuf::from("my.json")));
    assert_eq!(options.exclude, vec!["a", "b"]);
    assert!(options.apply_mode().write && options.apply_mode().upgrade_major);
}

#[test]
fn help_wins_over_everything_else() {
    assert_eq!(parse(&["--help"]), Ok(Command::Help));
    assert_eq!(parse(&["--write", "--upgrade-major", "--help"]), Ok(Command::Help));
}

#[test]
fn rejects_an_unknown_option() {
    assert_eq!(parse(&["projects", "--force"]), Err("unknown option: --force".to_string()));
}

#[test]
fn rejects_options_missing_their_value() {
    assert_eq!(parse(&["projects", "--config"]), Err("--config requires a value".to_string()));
    assert_eq!(parse(&["projects", "--exclude"]), Err("--exclude requires a value".to_string()));
}

#[test]
fn rejects_two_folders() {
    assert_eq!(parse(&["first", "second"]), Err("only one folder can be given: second".to_string()));
}

#[test]
fn rejects_a_missing_folder() {
    assert_eq!(parse(&["--write"]), Err("a folder must be specified".to_string()));
    assert_eq!(parse(&[]), Err("a folder must be specified".to_string()));
}

#[test]
fn upgrading_majors_requires_write() {
    let error = parse(&["projects", "--upgrade-major"]).unwrap_err();
    assert_eq!(error, "--upgrade-major must be used with --write");
}
