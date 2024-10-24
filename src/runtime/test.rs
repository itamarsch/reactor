use std::fs;
use std::io;
use std::process::Command;

#[test]
fn test_wat_files() -> io::Result<()> {
    let bin_dir = "./test";
    let out_dir = "./out";

    // Ensure the output directory exists
    fs::create_dir_all(out_dir)?;

    // Iterate over all entries in the bin directory
    for entry in fs::read_dir(bin_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Check if the file has a .wat extension
        let extension = path.extension().and_then(|s| s.to_str());
        if let Some(extension @ ("wasm" | "rs")) = extension {
            // Get the filename without extension
            let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap();

            // Define the output .wasm file path
            let wasm_output = format!("{}/{}.wasm", out_dir, file_stem);

            // Compile the .wat file to .wasm using wat2wasm

            let status = if extension == "wasm" {
                Command::new("wat2wasm")
                    .arg(&path)
                    .arg("-o")
                    .arg(&wasm_output)
                    .status()?
            } else if extension == "rs" {
                Command::new("rustc")
                    .arg("--target=wasm32-wasi")
                    .arg("-O")
                    .arg(&path)
                    .arg("-o")
                    .arg(&wasm_output)
                    .status()?
            } else {
                unreachable!()
            };

            assert!(
                status.success(),
                "wat2wasm failed on {:?} with exit code {:?}",
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
