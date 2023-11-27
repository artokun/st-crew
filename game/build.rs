use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=./schemas/*.fbs");

    let flatc_executable = if cfg!(windows) {
        "./flatc.exe"
    } else {
        "flatc"
    };

    let project_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let schemas_dir = project_dir.join("schemas");
    let rust_src_dir = project_dir.join("game/src");
    let rust_generated_path = rust_src_dir.join("generated");
    let ts_src_dir = project_dir.join("sdk/src");

    fs::remove_dir_all(&rust_generated_path).ok();

    let output = Command::new(flatc_executable)
        .arg("--gen-all")
        .arg("--rust")
        .arg("--rust-module-root-file")
        .arg("-o")
        .arg(rust_src_dir.join("generated"))
        .arg(schemas_dir.join("schema.fbs"))
        .output()
        .expect("failed to execute generate flatbuffer rust code");

    if !output.status.success() {
        panic!(
            "cargo:warning=failed to generate flatbuffer rust code: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let mut generated_mod_file = File::options()
        .read(true)
        .open(rust_generated_path.join("mod.rs"))
        .expect("failed to open generated mod file");

    let mut contents = String::new();

    contents.push_str("#![allow(dead_code, unused_imports, clippy::all)]\n\n");

    generated_mod_file
        .read_to_string(&mut contents)
        .expect("failed to read generated mod file");

    let mut generated_mod_file = File::options()
        .write(true)
        .truncate(true)
        .open(rust_generated_path.join("mod.rs"))
        .expect("failed to open generated mod file");

    generated_mod_file
        .write_all(contents.as_bytes())
        .expect("failed to write generated mod file");

    let output = Command::new(flatc_executable)
        .arg("--gen-object-api")
        .arg("--ts")
        .arg("-o")
        .arg(&ts_src_dir.join("generated"))
        .arg(schemas_dir.join("schema.fbs"))
        .output()
        .expect("failed to execute generate flatbuffer Typescript object api");

    if !output.status.success() {
        panic!(
            "failed to generate flatbuffer Typescript object api: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // uWebsocket support
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=uv");
    println!("cargo:rustc-link-lib=ssl");
    println!("cargo:rustc-link-lib=crypto");

    // Conditional linking for C++ standard library based on the target OS
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++"); // Use libc++ for macOS
    #[cfg(not(target_os = "macos"))]
    println!("cargo:rustc-link-lib=stdc++"); // Use libstdc++ for other systems
}
