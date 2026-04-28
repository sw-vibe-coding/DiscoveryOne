use std::fs;
use std::path::Path;
use std::process::ExitCode;

pub fn run_lower(file: &Path) -> ExitCode {
    match fs::read_to_string(file)
        .map_err(|err| err.to_string())
        .and_then(|source| d1_ir::lower_and_dump(&source))
    {
        Ok(output) => {
            print!("{output}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("d1 lower: {}: {err}", file.display());
            ExitCode::from(1)
        }
    }
}
