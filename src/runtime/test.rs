use std::fs;
use std::io;
use std::process::Command;

fn run_test_for_file(path_str: &str) -> io::Result<()> {
    let path = std::path::Path::new(path_str);
    let out_dir = "./out";

    // Ensure the output directory exists
    fs::create_dir_all(out_dir)?;

    let extension = path.extension().and_then(|s| s.to_str());
    if let Some(extension) = extension {
        if extension == "wat" || extension == "wasm" || extension == "rs" {
            // Get the filename without extension
            let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap();

            // Define the output .wasm file path
            let wasm_output = format!("{}/{}.wasm", out_dir, file_stem);

            // Compile the file to .wasm using appropriate tool
            let status = if extension == "wat" {
                Command::new("wat2wasm")
                    .arg(path)
                    .arg("-o")
                    .arg(&wasm_output)
                    .status()?
            } else if extension == "wasm" {
                fs::copy(path, &wasm_output)?;
                // Create a successful exit status
                Command::new("true").status()?
            } else if extension == "rs" {
                Command::new("rustc")
                    .arg("--target=wasm32-wasi")
                    .arg("-O")
                    .arg(path)
                    .arg("-o")
                    .arg(&wasm_output)
                    .status()?
            } else {
                unreachable!()
            };

            assert!(
                status.success(),
                "Compilation failed on {:?} with exit code {:?}",
                path,
                status.code()
            );

            // Run the compiled .wasm file with wasmtime, capturing stdout
            let wasmtime_output = Command::new("wasmtime").arg(&wasm_output).output()?; // Captures stdout, stderr, and exit status

            // Run your Rust application with the .wasm file as an argument, capturing stdout
            let cargo_output = Command::new("cargo")
                .arg("run")
                .arg("--quiet")
                .arg("--release")
                .arg("--")
                .arg(&wasm_output)
                .output()?; // Captures stdout, stderr, and exit status

            // Capture and compare the exit codes
            let wasmtime_code = wasmtime_output.status.code().unwrap_or(-1);
            let cargo_code = cargo_output.status.code().unwrap_or(-1);

            assert_eq!(
                wasmtime_code, cargo_code,
                "Exit codes differ for {:?}: wasmtime exit code {}, cargo run exit code {}",
                path, wasmtime_code, cargo_code
            );

            // Capture and compare the standard outputs
            let wasmtime_stdout = String::from_utf8_lossy(&wasmtime_output.stdout);
            let cargo_stdout = String::from_utf8_lossy(&cargo_output.stdout);

            assert_eq!(
                wasmtime_stdout, cargo_stdout,
                "Standard output differs for {:?}:\nwasmtime stdout:\n{}\ncargo run stdout:\n{}",
                path, wasmtime_stdout, cargo_stdout
            );
        }
    }

    Ok(())
}

// Include the generated test functions
include!(concat!(env!("OUT_DIR"), "/tests_generated.rs"));
