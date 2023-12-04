use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let sdk_generator = SdkGenerator::with_workspace(workspace_dir);

    println!(
        "cargo:rerun-if-changed={}",
        sdk_generator.schemas_dir.display()
    );

    sdk_generator.generate(Target::Typescript);
    sdk_generator.generate(Target::Go);
    sdk_generator.generate(Target::Python);
    sdk_generator.generate(Target::Dart);
}

struct SdkGenerator {
    schemas_dir: PathBuf,
    sdk_dir: PathBuf,
}

impl SdkGenerator {
    fn with_workspace(workspace_dir: impl AsRef<Path>) -> Self {
        let workspace_dir = workspace_dir.as_ref();
        let schemas_dir = workspace_dir.join("schemas");
        let sdk_dir = workspace_dir.join("sdk");

        Self {
            schemas_dir: schemas_dir.to_owned(),
            sdk_dir: sdk_dir.to_owned(),
        }
    }

    fn generate(&self, target: Target) {
        let mut cmd: Command;

        if cfg!(windows) {
            cmd = Command::new("cmd");
            cmd.args(["/c", "quicktype"]);
        } else {
            cmd = Command::new("quicktype");
        }

        cmd.current_dir(&self.schemas_dir)
            .arg("--src-lang")
            .arg("schema")
            .arg("--src")
            .arg(self.schemas_dir.join("schema.json"))
            .arg("-o")
            .arg(self.sdk_dir.join(target.output()))
            .arg("--no-combine-classes")
            .arg("--lang")
            .arg(target.lang());

        for arg in target.args() {
            cmd.arg(arg);
        }

        match cmd.output() {
            Ok(output) => {
                if !output.status.success() {
                    println!(
                        "cargo:warning=failed to generate {:?} sdk: {}",
                        target,
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }

            Err(e) => println!(
                "cargo:warning=failed to execute quicktype for {:?} sdk: {}",
                target, e
            ),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum Target {
    Rust,
    Typescript,
    Go,
    Python,
    Dart,
}

impl Target {
    fn lang(&self) -> &str {
        match self {
            Target::Rust => "rust",
            Target::Typescript => "typescript-zod",
            Target::Go => "go",
            Target::Python => "python",
            Target::Dart => "dart",
        }
    }

    fn output(&self) -> PathBuf {
        match self {
            Target::Rust => PathBuf::new().join("rust").join("src").join("generated.rs"),
            Target::Typescript => PathBuf::new().join("ts").join("src").join("generated.ts"),
            Target::Go => PathBuf::new().join("go").join("generated"),
            Target::Python => PathBuf::new().join("python").join("generated.py"),
            Target::Dart => PathBuf::new()
                .join("dart")
                .join("lib")
                .join("generated.dart"),
        }
    }

    fn args(&self) -> &[&'static str] {
        match self {
            Target::Rust => &[
                "--density",
                "normal",
                "--visibility",
                "public",
                "--derive-debug",
                "--derive-partial-eq",
            ],

            Target::Typescript => &[],

            Target::Go => &["--package", "st_crew_sdk", "--multi-file-output"],

            Target::Python => &[],

            Target::Dart => &[
                "--coders-in-class",
                // "--final-props",
                // "--copy-with",
                "--use-freezed",
            ],
        }
    }
}
