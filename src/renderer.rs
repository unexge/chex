use std::collections::HashMap;
use std::fmt;
use std::io::Write;

use termion::{clear, color, cursor, style};

use crate::group::Group;

pub struct Renderer<'a> {
    groups: Vec<Group<'a>>,
    focused: Option<usize>,
    expanded: HashMap<usize, bool>,
}

impl<'a> Renderer<'a> {
    pub fn new(groups: Vec<Group<'a>>) -> Self {
        Self {
            groups,
            focused: None,
            expanded: HashMap::new(),
        }
    }

    pub fn focus_next(&mut self) {
        if self.groups.is_empty() {
            return;
        }

        self.focused = match self.focused.take() {
            Some(x) if x == self.groups.len() - 1 => Some(0),
            Some(x) => Some(x + 1),
            None => Some(0),
        }
    }

    pub fn focus_prev(&mut self) {
        if self.groups.is_empty() {
            return;
        }

        self.focused = match self.focused.take() {
            Some(0) => Some(self.groups.len() - 1),
            Some(x) => Some(x - 1),
            None => Some(self.groups.len() - 1),
        }
    }

    pub fn expand_current(&mut self) {
        match self.focused {
            Some(i) => {
                let entry = self.expanded.entry(i).or_insert(false);
                *entry = !*entry;
            }
            None => {}
        }
    }

    pub fn collapse_current(&mut self) {
        match self.focused {
            Some(i) => {
                let entry = self.expanded.entry(i).or_insert(true);
                *entry = !*entry;
            }
            None => {}
        }
    }

    pub fn render(&self, output: &mut impl Write) {
        write!(
            output,
            "{}{}{}",
            clear::All,
            cursor::Goto(1, 1),
            cursor::Hide
        )
        .unwrap();

        for (i, group) in self.groups.iter().enumerate() {
            let is_focused = matches!(self.focused, Some(x) if x == i);
            let is_expanded = self.expanded.get(&i).unwrap_or(&false);
            self.render_group(output, group, is_focused, *is_expanded);
        }

        output.flush().unwrap();
    }

    fn render_group(
        &self,
        output: &mut impl Write,
        group: &Group,
        is_focused: bool,
        is_expanded: bool,
    ) {
        let mut lines = group.inner().iter();
        let color: Box<dyn fmt::Display> = match group {
            Group::Error(_) => Box::new(color::Fg(color::Red)),
            Group::Warning(_) => Box::new(color::Fg(color::Yellow)),
        };

        write!(output, "\r\n").unwrap();

        let summary = lines.next().unwrap_or(&"");
        if is_focused {
            write!(
                output,
                "\t{}{}{}{}{}\r\n",
                style::Underline,
                color,
                summary,
                color::Fg(color::Reset),
                style::Reset
            )
            .unwrap();
        } else {
            write!(
                output,
                "\t{}{}{}\r\n",
                color,
                summary,
                color::Fg(color::Reset),
            )
            .unwrap();
        }

        if is_expanded {
            for line in lines {
                write!(output, "\t{}\r\n", line).unwrap();
            }
        }
    }
}
