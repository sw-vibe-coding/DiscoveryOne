use std::fs;
use std::path::Path;
use std::process::ExitCode;

pub fn run_interp(file: &Path, inputs: &[String]) -> ExitCode {
    match fs::read_to_string(file)
        .map_err(|err| err.to_string())
        .and_then(|source| d1_interp::run_and_dump(&source, inputs))
    {
        Ok(output) => {
            print!("{output}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("d1 interp: {}: {err}", file.display());
            ExitCode::from(1)
        }
    }
}

pub fn run_wasm(file: &Path, inputs: &[String]) -> ExitCode {
    match fs::read_to_string(file)
        .map_err(|err| err.to_string())
        .and_then(|source| d1_emit_wasm::run_and_dump(&source, inputs))
    {
        Ok(output) => {
            print!("{output}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("d1 run: {}: {err}", file.display());
            ExitCode::from(1)
        }
    }
}
