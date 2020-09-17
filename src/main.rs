#![feature(peekable_next_if)]

use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::str;

use termion::cursor;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod group;
mod renderer;
use renderer::Renderer;

fn main() {
    let output = Command::new("cargo")
        .arg("check")
        .arg("--color=never")
        .output()
        .expect("failed to execute `cargo check`");

    let stderr = str::from_utf8(&output.stderr).expect("non utf8 output");
    if stderr.trim().starts_with("Finished") {
        // exited without errors
        return;
    }

    let groups = group::parse(stderr);
    let mut renderer = Renderer::new(groups);

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    renderer.render(&mut stdout);
    for evt in stdin.events() {
        match evt.unwrap() {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Up) => {
                renderer.focus_prev();
            }
            Event::Key(Key::Down) => {
                renderer.focus_next();
            }
            Event::Key(Key::Left) => {
                renderer.collapse_current();
            }
            Event::Key(Key::Right) => {
                renderer.expand_current();
            }
            _ => {}
        }
        renderer.render(&mut stdout);
    }

    write!(stdout, "{}", cursor::Show).unwrap();
}
