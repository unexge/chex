use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io::{stdin, stdout, Write};

use cargo_metadata::diagnostic::{Diagnostic, DiagnosticLevel};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style};

pub struct Renderer {
    diagnostics: Vec<Diagnostic>,
    focused: Option<usize>,
    expanded: HashMap<usize, bool>,
}

impl Renderer {
    pub fn new(diagnostics: Vec<Diagnostic>) -> Self {
        Self {
            diagnostics,
            focused: None,
            expanded: HashMap::new(),
        }
    }

    pub fn start(mut self) -> Result<(), Box<dyn Error>> {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        self.render(&mut stdout);

        for evt in stdin.events() {
            match evt? {
                Event::Key(Key::Char('q')) => break,
                Event::Key(Key::Up) => {
                    self.focus_prev();
                }
                Event::Key(Key::Down) => {
                    self.focus_next();
                }
                Event::Key(Key::Left) => {
                    self.collapse_current();
                }
                Event::Key(Key::Right) => {
                    self.expand_current();
                }
                _ => {}
            }

            self.render(&mut stdout);
        }

        write!(stdout, "{}", cursor::Show).unwrap();
        Ok(())
    }

    fn render(&self, output: &mut impl Write) {
        write!(
            output,
            "{}{}{}",
            clear::All,
            cursor::Goto(1, 1),
            cursor::Hide
        )
        .unwrap();

        for (i, diagnostic) in self.diagnostics.iter().enumerate() {
            let is_focused = matches!(self.focused, Some(x) if x == i);
            let is_expanded = self.expanded.get(&i).unwrap_or(&false);
            self.render_diagnostic(output, diagnostic, is_focused, *is_expanded);
        }

        output.flush().unwrap();
    }

    fn render_diagnostic(
        &self,
        output: &mut impl Write,
        diagnostic: &Diagnostic,
        is_focused: bool,
        is_expanded: bool,
    ) {
        let rendered = diagnostic
            .rendered
            .clone()
            .unwrap_or_else(|| "".to_string());
        let mut lines = rendered.lines();
        let color: Box<dyn fmt::Display> = match diagnostic.level {
            DiagnosticLevel::Error | DiagnosticLevel::Ice => Box::new(color::Fg(color::Red)),
            DiagnosticLevel::Warning => Box::new(color::Fg(color::Yellow)),
            _ => Box::new(color::Fg(color::Reset)),
        };

        write!(output, "\r\n").unwrap();

        let summary = lines.next().unwrap_or(&"");
        if is_focused {
            write!(
                output,
                "\t{}{}{}{}{}{}\r\n",
                style::Underline,
                style::Bold,
                color,
                summary,
                color::Fg(color::Reset),
                style::Reset
            )
            .unwrap();
        } else {
            write!(
                output,
                "\t{}{}{}{}{}\r\n",
                style::Bold,
                color,
                summary,
                color::Fg(color::Reset),
                style::Reset,
            )
            .unwrap();
        }

        if is_expanded {
            for line in lines {
                write!(output, "\t{}\r\n", line).unwrap();
            }
        }
    }

    fn focus_next(&mut self) {
        if self.diagnostics.is_empty() {
            return;
        }

        self.focused = match self.focused.take() {
            Some(x) if x == self.diagnostics.len() - 1 => Some(0),
            Some(x) => Some(x + 1),
            None => Some(0),
        }
    }

    fn focus_prev(&mut self) {
        if self.diagnostics.is_empty() {
            return;
        }

        self.focused = match self.focused.take() {
            Some(0) => Some(self.diagnostics.len() - 1),
            Some(x) => Some(x - 1),
            None => Some(self.diagnostics.len() - 1),
        }
    }

    fn expand_current(&mut self) {
        if let Some(i) = self.focused {
            self.expanded.insert(i, true);
        }
    }

    fn collapse_current(&mut self) {
        if let Some(i) = self.focused {
            self.expanded.remove(&i);
        }
    }
}
