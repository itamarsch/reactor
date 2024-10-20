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
        if path.extension().and_then(|s| s.to_str()) == Some("wat") {
            // Get the filename without extension
            let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap();

            // Define the output .wasm file path
            let wasm_output = format!("{}/{}.wasm", out_dir, file_stem);

            // Compile the .wat file to .wasm using wat2wasm
            let status = Command::new("wat2wasm")
                .arg(&path)
                .arg("-o")
                .arg(&wasm_output)
                .status()?;

            assert!(
                status.success(),
                "wat2wasm failed on {:?} with exit code {:?}",
                path,
                status.code()
            );

            // Run the compiled .wasm file with wasmtime
            let wasmtime_status = Command::new("wasmtime").arg(&wasm_output).status()?;

            // Run your Rust application with the .wasm file as an argument
            let cargo_status = Command::new("cargo")
                .arg("run")
                .arg("--quiet")
                .arg("--")
                .arg(&wasm_output)
                .status()?;

            // Capture and compare the exit codes
            let wasmtime_code = wasmtime_status.code().unwrap_or(-1);
            let cargo_code = cargo_status.code().unwrap_or(-1);

            assert_eq!(
                wasmtime_code, cargo_code,
                "Exit codes differ for {:?}: wasmtime exit code {}, cargo run exit code {}",
                path, wasmtime_code, cargo_code
            );

            assert_eq!();
        }
    }

    Ok(())
}
