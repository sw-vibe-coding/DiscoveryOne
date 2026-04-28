//! `d1` -- command-line driver for DiscoveryOne.
//!
//! Subcommand surface defined in `docs/design.md` section 9.
//! In the M0 scaffold (`discoveryone-scaffold`) every
//! subcommand starts as a stub: it parses arguments, prints
//! `not yet implemented` to stderr, and exits 1 until its
//! milestone lands.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

mod commands;
mod interp;
mod lower;
use commands::{run_check, run_lex, run_parse, run_source};
use interp::{run_interp, run_wasm};
use lower::run_lower;

const ABOUT: &str = "DiscoveryOne language driver. Test-harness CLI; see docs/design.md.";
const LONG_ABOUT: &str = "\
DiscoveryOne language driver. Test-harness CLI; see docs/design.md.

AI CODING AGENT INSTRUCTIONS:

Use d1 as the stable command-line surface for regression tests.
Prefer reg-rs fixtures under work/reg-rs when adding behavior.
Subcommands other than lex are scaffolded and may exit with
\"not yet implemented\" until their milestone lands.
";
const VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "\nCopyright: Copyright (c) 2026 Michael A Wright",
    "\nLicense: MIT",
    "\nRepository: https://github.com/sw-vibe-coding/DiscoveryOne",
    "\nBuild Host: ",
    env!("BUILD_HOST"),
    "\nBuild Commit: ",
    env!("BUILD_COMMIT"),
    "\nBuild Time: ",
    env!("BUILD_TIME"),
);

#[derive(Parser)]
#[command(
    name = "d1",
    version = VERSION,
    about = ABOUT,
    long_about = LONG_ABOUT,
    disable_help_subcommand = true
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Dump the token stream for FILE (one token per line).
    Lex {
        /// Source file (.d1 or .d1.json).
        file: PathBuf,
    },
    /// Dump the AST for FILE as a deterministic s-expression.
    Parse {
        /// Source file.
        file: PathBuf,
    },
    /// Project FILE onto FACE; print the 2D token grid.
    Face {
        /// Source file.
        file: PathBuf,
        /// Face name: front, left, right, top, bottom, rear.
        #[arg(long)]
        face: String,
    },
    /// Run the facet checker over FILE.
    Check {
        /// Source file.
        file: PathBuf,
    },
    /// Print the stack IR for FILE.
    Lower {
        /// Source file.
        file: PathBuf,
    },
    /// Emit FILE as a .wasm module.
    EmitWasm {
        /// Source file.
        file: PathBuf,
        /// Output path for the .wasm bytes.
        #[arg(short = 'o', long = "out")]
        out: PathBuf,
    },
    /// Execute FILE via the WASM runtime (wasmtime).
    Run {
        /// Source file.
        file: PathBuf,
        /// Inputs as KEY=VALUE pairs (comma-separated or repeated).
        #[arg(long, value_delimiter = ',')]
        inputs: Vec<String>,
    },
    /// Execute FILE via the reference interpreter.
    Interp {
        /// Source file.
        file: PathBuf,
        /// Inputs as KEY=VALUE pairs (comma-separated or repeated).
        #[arg(long, value_delimiter = ',')]
        inputs: Vec<String>,
    },
    /// Round-trip FILE between layered text and JSON.
    Normalize {
        /// Source file.
        file: PathBuf,
    },
    /// Render FACE of FILE as deterministic ASCII (for reg-rs).
    FacetSnapshot {
        /// Source file.
        file: PathBuf,
        /// Face name: front, left, right, top, bottom, rear.
        #[arg(long)]
        face: String,
    },
}

impl Cmd {
    fn name(&self) -> &'static str {
        match self {
            Cmd::Lex { .. } => "lex",
            Cmd::Parse { .. } => "parse",
            Cmd::Face { .. } => "face",
            Cmd::Check { .. } => "check",
            Cmd::Lower { .. } => "lower",
            Cmd::EmitWasm { .. } => "emit-wasm",
            Cmd::Run { .. } => "run",
            Cmd::Interp { .. } => "interp",
            Cmd::Normalize { .. } => "normalize",
            Cmd::FacetSnapshot { .. } => "facet-snapshot",
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Lex { file } => run_lex(&file),
        Cmd::Parse { file } => run_parse(&file),
        Cmd::Check { file } => run_check(&file),
        Cmd::Lower { file } => run_lower(&file),
        Cmd::Run { file, inputs } => run_wasm(&file, &inputs),
        Cmd::Interp { file, inputs } => run_interp(&file, &inputs),
        Cmd::Face { file, face } => run_source(&file, Some(&face), "face"),
        Cmd::FacetSnapshot { file, face } => run_source(&file, Some(&face), "facet-snapshot"),
        Cmd::Normalize { file } => run_source(&file, None, "normalize"),
        cmd => {
            eprintln!("d1 {}: not yet implemented", cmd.name());
            ExitCode::from(1)
        }
    }
}
