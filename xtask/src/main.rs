use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

type DynError = Box<dyn std::error::Error>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task.as_ref().map(|it| it.as_str()) {
        Some("build-dev") => build_dev()?,
        Some("run-dev") => run_dev()?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "\
Tasks:
build-dev       Build dev files
run-dev         Run dev server
"
    )
}

fn build_dev() -> Result<(), DynError> {
    println!("Running wasm-pack...");
    let status = Command::new("wasm-pack")
        .current_dir(project_root())
        .args(&[
            "build",
            "wasm-module/",
            "--dev",
            "--out-dir",
            "../target/wasm-dist-dev",
            "--target",
            "web",
        ])
        .status()?;
    if !status.success() {
        Err("wasm-pack failed!")?;
    }
    Ok(())
}

fn run_dev() -> Result<(), DynError> {
    build_dev()?;

    println!("Running dev-server...");
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir(project_root())
        .args(&["run", "--package", "dev-server"])
        .status()?;
    if !status.success() {
        Err("cargo run failed!")?;
    }
    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
