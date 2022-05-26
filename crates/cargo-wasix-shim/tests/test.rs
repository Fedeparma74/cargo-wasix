use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn me() -> PathBuf {
    let mut me = std::env::current_exe().unwrap();
    me.pop();
    me.pop();
    me.push("cargo-wasix-shim");
    me.set_extension(std::env::consts::EXE_EXTENSION);
    return me;
}

fn case() -> (tempfile::TempDir, PathBuf) {
    let td = tempfile::TempDir::new().unwrap();
    let mut path = td.path().join("cargo-wasix");
    path.set_extension(std::env::consts::EXE_EXTENSION);
    fs::copy(me(), &path).unwrap();
    (td, path)
}

#[test]
fn smoke() {
    let (_td, path) = case();
    let before = fs::read(&path).unwrap();
    let output = Command::new(&path).output().unwrap();
    println!("{:#?}", output);
    assert!(output.status.success());
    let after = fs::read(&path).unwrap();
    assert!(after != before);
    if cfg!(windows) {
        assert!(path.with_file_name(".cargo-wasix.exe").exists());
    } else {
        assert!(!path.with_file_name(".cargo-wasix").exists());
    }
}

#[test]
fn pass_args() {
    let (_td, path) = case();
    let output = Command::new(&path).arg("--help").output().unwrap();
    println!("{:#?}", output);
    assert!(output.status.success());
}

#[test]
fn run_twice() {
    let (_td, path) = case();
    let output = Command::new(&path).output().unwrap();
    println!("{:#?}", output);
    assert!(output.status.success());
    let output = Command::new(&path).output().unwrap();
    println!("{:#?}", output);
    assert!(output.status.success());
    assert!(!path.with_file_name(".cargo-wasix").exists());
    assert!(!path.with_file_name(".cargo-wasix.exe").exists());
}
