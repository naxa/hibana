use std::process::Command;
use std::path::Path;
use std::fs::remove_dir_all;

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        remove_dir_all(Path::new(NATORI)).unwrap();
    }
}

const NEW_WITHOUT_NAME: &str = "error: The following required arguments were not provided:
    <name>

USAGE:
    hibana new <name>

For more information try --help
";

const BUILD_WITHOUT_PROJECT: &str = "\u{1b}[1;31merror\u{1b}[0m: \
contents dir is not found. hint: execute \'hibana new project_name\'
";

const NATORI: &str = "natori";

#[test]
fn test_new_without_name() {
    let output = Command::new("target/debug/hibana")
        .arg("new")
        .output()
        .expect("failed to run new command");

    assert_eq!(String::from_utf8_lossy(&output.stderr), NEW_WITHOUT_NAME);
}

#[test]
fn test_cmd_new() {
    let _cleanup = Cleanup;
    Command::new("target/debug/hibana")
        .arg("new")
        .arg(NATORI)
        .output()
        .expect("failed to run new command");

    let path = Path::new(NATORI);
    assert!(path.exists());
    assert!(path.join("contents").exists());
    assert!(path.join("layouts").exists());
    assert!(path.join("public").exists());
    assert!(path.join("assets").exists());
    assert!(path.join("config.toml").exists());
    assert!(path.join("contents").join("index.md").exists());
}

#[test]
fn test_build_without_new() {
    let output = Command::new("target/debug/hibana")
        .arg("build")
        .output()
        .expect("failed to run build command");

    assert_eq!(String::from_utf8_lossy(&output.stdout), BUILD_WITHOUT_PROJECT);
}

#[test]
fn test_serve_without_new() {
    let output = Command::new("target/debug/hibana")
        .arg("serve")
        .output()
        .expect("failed to run build command");

    assert_eq!(String::from_utf8_lossy(&output.stdout), BUILD_WITHOUT_PROJECT);
}
