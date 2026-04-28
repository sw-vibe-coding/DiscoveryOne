use std::fs;
use std::path::Path;
use std::process::ExitCode;

pub fn run_lex(file: &Path) -> ExitCode {
    match fs::read_to_string(file) {
        Ok(source) => {
            print!("{}", d1_lex::dump_tokens(&d1_lex::lex(&source)));
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("d1 lex: {}: {err}", file.display());
            ExitCode::from(1)
        }
    }
}

pub fn run_parse(file: &Path) -> ExitCode {
    match fs::read_to_string(file)
        .map_err(|err| err.to_string())
        .and_then(|source| d1_parse::parse_and_dump(&source))
    {
        Ok(output) => {
            print!("{output}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("d1 parse: {}: {err}", file.display());
            ExitCode::from(1)
        }
    }
}

pub fn run_check(file: &Path) -> ExitCode {
    match fs::read_to_string(file)
        .map_err(|err| err.to_string())
        .and_then(|source| d1_check::check_and_dump(&source))
    {
        Ok(output) => {
            print!("{output}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("d1 check: {}: {err}", file.display());
            ExitCode::from(1)
        }
    }
}

pub fn run_source(file: &Path, face: Option<&str>, command: &str) -> ExitCode {
    match fs::read_to_string(file)
        .map_err(|err| err.to_string())
        .and_then(|source| d1_source::emit_layered(&source, face))
    {
        Ok(output) => {
            print!("{output}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("d1 {command}: {}: {err}", file.display());
            ExitCode::from(1)
        }
    }
}
