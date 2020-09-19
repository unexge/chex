use std::error::Error;

mod diagnostics;
mod renderer;

fn main() -> Result<(), Box<dyn Error>> {
    let diagnostics = diagnostics::from_cargo_check(".")?;
    if diagnostics.len() == 0 {
        return Ok(());
    }

    let renderer = renderer::Renderer::new(diagnostics);
    renderer.start()?;
    Ok(())
}
