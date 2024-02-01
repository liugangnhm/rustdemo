use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("commit_id.rs");
    let mut f = File::create(&dest_path).unwrap();

    let commit = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .expect("Failed to execute git command");
    let commit = String::from_utf8(commit.stdout).expect("Invalid utf8 string");

    let output = format!(
        r#"pub const CURRENT_COMMIT_ID : &'static str = "{}"; "#,
        commit.trim()
    );

    f.write_all(output.as_bytes()).unwrap();

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("generating c bindings header file......");
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        // .with_config(Config::from_file("lib.toml").unwrap())
        // .exclude_item("Box")
        .with_language(cbindgen::Language::C)
        .with_cpp_compat(true)
        .generate()
        .expect("Unable to generate c bindings")
        .write_to_file("bindings.h");

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // generate cpython bindings
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        // .with_config(Config::from_file("lib.toml").unwrap())
        // .exclude_item("Box")
        .with_language(cbindgen::Language::Cython)
        .with_cpp_compat(true)
        .generate()
        .expect("Unable to generate cython bindings")
        .write_to_file("bindings.pyx");
}
