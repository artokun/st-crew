use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let script_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("generate.sh");
    let output = Command::new("sh")
        .arg(script_path)
        .arg(&out_dir)
        .output()
        .expect("Failed to execute shell script");

    if !output.status.success() {
        panic!(
            "Shell script failed with output:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let generated_path = Path::new(&out_dir).join("generated");
    let mut new_content = String::from("mod generated {\n");

    let entries: Vec<_> = fs::read_dir(generated_path).unwrap().collect();

    for entry in &entries {
        let entry = entry.as_ref().unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().unwrap() == "rs" {
            // if path is generated.rs, skip it
            if path.file_name().unwrap() == "generated.rs" {
                continue;
            }
            let module_name = path.file_stem().unwrap().to_str().unwrap();
            new_content.push_str(&format!(
                "#[allow(dead_code, unused_imports, clippy::all)]\npub mod {};\n",
                module_name
            ));
        }
    }
    new_content.push_str("\npub use message_generated::*;\n}");

    let dest_path = Path::new(&out_dir).join("generated.rs");
    fs::write(dest_path, new_content).unwrap();

    println!("cargo:rerun-if-changed=schemas/*.fbs");
}
