use std::error::Error;
use std::process::Command;

use cargo_metadata::{
    diagnostic::{Diagnostic, DiagnosticLevel},
    Message,
};

pub fn from_cargo_check(project_dir: &str) -> Result<Vec<Diagnostic>, Box<dyn Error>> {
    let output = Command::new("cargo")
        .current_dir(project_dir)
        .args(&["check", "--message-format=json"])
        .output()?;

    Ok(Message::parse_stream(output.stdout.as_slice())
        .filter_map(|m| match m {
            Ok(Message::CompilerMessage(cm)) => Some(cm.message),
            _ => None,
        })
        .filter(|d| !matches!(d.level, DiagnosticLevel::Unknown))
        // TODO: is there a better way to skip
        // `error: aborting due to 2 previous errors` and `warning: 1 warning emitted`
        // kind of outputs?
        .filter(|d| {
            !matches!(d.level, DiagnosticLevel::Error | DiagnosticLevel::Warning)
                || d.code.is_some()
        })
        .collect())
}
