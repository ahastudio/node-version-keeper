use std::process::Command;

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_node-version-keeper"))
}

#[test]
fn prints_the_usage_for_help() {
    let output = binary().arg("--help").output().expect("binary should run");
    let standard_output = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(
        standard_output.starts_with("Usage: node-version-keeper <folder> [options]"),
        "{standard_output}"
    );
}

#[test]
fn exits_with_code_two_when_no_folder_is_given() {
    let output = binary().output().expect("binary should run");
    let standard_error = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2));
    assert!(standard_error.contains("Error: a folder must be specified"), "{standard_error}");
}
