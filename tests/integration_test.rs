use std::process::Command;
use std::path::Path;
use std::fs::{create_dir, File, remove_dir_all};

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        remove_dir_all(Path::new("natori")).unwrap();
        println!("hoge");
    }
}

static NEW_WITHOUT_NAME: &'static str = "error: The following required arguments were not provided:
    <name>

USAGE:
    hibana new <name>

For more information try --help
";

static BUILD_WITHOUT_PROJECT: &'static str = "\u{1b}[1;31merror\u{1b}[0m: \
contents dir is not found. hint: execute \'hibana new project_name\'
";

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
        .arg("natori")
        .output()
        .expect("failed to run new command");

    let path = Path::new("natori");
    assert_eq!(path.exists(), true);
    assert_eq!(path.join("contents").exists(), true);
    assert_eq!(path.join("layouts").exists(), true);
    assert_eq!(path.join("public").exists(), true);
    assert_eq!(path.join("assets").exists(), true);
    assert_eq!(path.join("config.toml").exists(), true);
    assert_eq!(path.join("contents").join("index.md").exists(), true);
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