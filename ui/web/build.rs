use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    emit_git_rerun_hints();

    let host = command_stdout("hostname", &[]).unwrap_or_else(|| "unknown".to_string());
    println!("cargo:rustc-env=BUILD_HOST={host}");

    let commit = command_stdout("git", &["rev-parse", "--short", "HEAD"])
        .unwrap_or_else(|| "unknown".to_string());
    println!("cargo:rustc-env=BUILD_COMMIT={commit}");

    let timestamp =
        command_stdout("date", &["-u", "+%Y-%m-%dT%H:%M:%SZ"]).unwrap_or_else(fallback_timestamp);
    println!("cargo:rustc-env=BUILD_TIME={timestamp}");
}

fn emit_git_rerun_hints() {
    println!("cargo:rerun-if-changed=../../.git/HEAD");
    if let Ok(head) = std::fs::read_to_string("../../.git/HEAD")
        && let Some(ref_name) = head.trim().strip_prefix("ref: ")
    {
        println!("cargo:rerun-if-changed=../../.git/{ref_name}");
    }
}

fn command_stdout(command: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(command).args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let value = String::from_utf8(output.stdout).ok()?.trim().to_string();
    (!value.is_empty()).then_some(value)
}

fn fallback_timestamp() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    format!("{seconds}s since UNIX epoch")
}
