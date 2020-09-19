use std::error::Error;

mod diagnostics;
mod renderer;

fn main() -> Result<(), Box<dyn Error>> {
    let diagnostics = diagnostics::from_cargo_check(".")?;
    if diagnostics.is_empty() {
        return Ok(());
    }

    let renderer = renderer::Renderer::new(diagnostics);
    renderer.start()?;
    Ok(())
}
