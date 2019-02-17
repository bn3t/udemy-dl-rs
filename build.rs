extern crate chrono;

use chrono::Local;
use std::process::Command;

fn main() {
    let unknown: Result<String, &str> = Ok("unknown".to_string());
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output();
    let git_hash = match output {
        Ok(output) => String::from_utf8(output.stdout).unwrap(),
        Err(_) => std::env::var("TRAVIS_COMMIT").or(unknown).unwrap(),
    };

    println!("cargo:rustc-env=GIT_COMMIT={}", git_hash);

    let date = Local::now();
    println!("cargo:rustc-env=BUILD_DATE={}", date.format("%Y-%m-%d"));
}
