use std::error::Error;

use cargo_metadata::diagnostic::DiagnosticLevel;

use chex::diagnostics;

#[test]
fn getting_diagnostics() -> Result<(), Box<dyn Error>> {
    let diagnostics = diagnostics::from_cargo_check("tests/simple_project")?;

    assert_eq!(
        diagnostics
            .into_iter()
            .map(|d| (stringify_level(d.level), d.message))
            .collect::<Vec<_>>(),
        vec![
            (
                stringify_level(DiagnosticLevel::Error),
                "mismatched types".to_string()
            ),
            (
                stringify_level(DiagnosticLevel::Error),
                "can't compare `{integer}` with `&str`".to_string()
            ),
        ],
    );

    Ok(())
}

fn stringify_level(level: DiagnosticLevel) -> String {
    match level {
        DiagnosticLevel::Ice => "ice".to_string(),
        DiagnosticLevel::Error => "error".to_string(),
        DiagnosticLevel::Warning => "warning".to_string(),
        DiagnosticLevel::Note => "note".to_string(),
        DiagnosticLevel::Help => "help".to_string(),
        DiagnosticLevel::Unknown => "unknown".to_string(),
    }
}
