use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let bin_dir = "./test";
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(&out_dir).join("tests_generated.rs");
    let mut f = fs::File::create(dest_path)?;

    // Iterate over all entries in the bin directory
    for entry in fs::read_dir(bin_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Check if the file has a .wat, .wasm, or .rs extension
        let extension = path.extension().and_then(|s| s.to_str());
        if let Some(extension) = extension {
            if extension == "wat" || extension == "wasm" || extension == "rs" {
                // Get the filename without extension
                let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap();

                // Sanitize the file_stem to be used as a function name
                let sanitized_file_stem = file_stem.replace(|c: char| !c.is_alphanumeric(), "_");

                // Generate a test function for this file
                writeln!(f, "#[test]")?;
                writeln!(f, "fn test_{}() {{", sanitized_file_stem)?;
                writeln!(
                    f,
                    "    run_test_for_file({:?}).unwrap();",
                    path.display().to_string()
                )?;
                writeln!(f, "}}")?;
            }
        }
    }

    Ok(())
}
