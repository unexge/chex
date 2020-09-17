#[derive(Debug, PartialEq)]
pub enum Group<'a> {
    Warning(Vec<&'a str>),
    Error(Vec<&'a str>),
}

impl Group<'_> {
    pub fn inner(&self) -> &Vec<&str> {
        match self {
            Group::Warning(v) => v,
            Group::Error(v) => v,
        }
    }
}

pub fn parse(input: &str) -> Vec<Group> {
    let mut lines = input.trim().lines().peekable();

    // first line might be "Checking xxx..." skip it if that is the case
    lines.next_if(|l| l.trim().starts_with("Checking"));

    let groups = &mut vec![vec![]];
    // split input text by empty lines
    let groups = lines.fold(groups, |acc, curr| {
        if curr.trim().is_empty() {
            acc.push(vec![]);
        } else {
            acc.last_mut().unwrap().push(curr);
        }
        acc
    });

    groups
        .into_iter()
        .filter_map(|lines| match lines.first() {
            Some(line) if line.starts_with("warning:") => Some(Group::Warning(lines.to_vec())),
            Some(line) if line.starts_with("error[") => Some(Group::Error(lines.to_vec())),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{parse, Group};

    #[test]
    fn parses_input() {
        let groups = parse(
            r#"
    Checking chex v0.1.0 (/chex)
warning: unused import: `std::sync::Arc`
 --> src/main.rs:1:5
  |
1 | use std::sync::Arc;
  |     ^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

error[E0277]: can't compare `std::vec::Vec<{integer}>` with `{integer}`
 --> src/main.rs:6:7
  |
6 |     x == y;
  |       ^^ no implementation for `std::vec::Vec<{integer}> == {integer}`
  |
  = help: the trait `std::cmp::PartialEq<{integer}>` is not implemented for `std::vec::Vec<{integer}>`
  = note: required because of the requirements on the impl of `std::cmp::PartialEq<std::vec::Vec<{integer}>>` for `std::vec::Vec<std::vec::Vec<{integer}>>`

error[E0308]: mismatched types
 --> src/main.rs:3:13
  |
3 | fn foo() -> bool {
  |    ---      ^^^^ expected `bool`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
...
6 |     x == y;
  |           - help: consider removing this semicolon

error: aborting due to 2 previous errors; 1 warning emitted

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `chex`.

To learn more, run the command again with --verbose.
"#,
        );

        assert_eq!(
            groups,
            vec![
                Group::Warning(
                    r#"
warning: unused import: `std::sync::Arc`
 --> src/main.rs:1:5
  |
1 | use std::sync::Arc;
  |     ^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default"#.trim().lines().collect()
                ),
                Group::Error(
                    r#"
error[E0277]: can't compare `std::vec::Vec<{integer}>` with `{integer}`
 --> src/main.rs:6:7
  |
6 |     x == y;
  |       ^^ no implementation for `std::vec::Vec<{integer}> == {integer}`
  |
  = help: the trait `std::cmp::PartialEq<{integer}>` is not implemented for `std::vec::Vec<{integer}>`
  = note: required because of the requirements on the impl of `std::cmp::PartialEq<std::vec::Vec<{integer}>>` for `std::vec::Vec<std::vec::Vec<{integer}>>`"#.trim().lines().collect()
                ),
                Group::Error(
                    r#"
error[E0308]: mismatched types
 --> src/main.rs:3:13
  |
3 | fn foo() -> bool {
  |    ---      ^^^^ expected `bool`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
...
6 |     x == y;
  |           - help: consider removing this semicolon
"#.trim().lines().collect()
                ),
            ]
        );
    }
}
