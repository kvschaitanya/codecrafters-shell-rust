use rustyline::{
    Helper, completion::Completer, highlight::Highlighter, hint::Hinter, validate::Validator,
};

use crate::BUILTIN_COMMANDS;

pub struct ShellCompleter {}

impl Completer for ShellCompleter {
    type Candidate = String;
    fn complete(
        &self, // FIXME should be `&mut self`
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let word = line.split(' ').next_back().unwrap();

        Ok((
            pos - word.len(),
            BUILTIN_COMMANDS
                .iter()
                .filter(|command| command.starts_with(word))
                .map(|&cmd| cmd.to_owned())
                .collect(),
        ))
    }
}
impl Hinter for ShellCompleter {
    type Hint = String;
}
impl Highlighter for ShellCompleter {}
impl Validator for ShellCompleter {}
impl Helper for ShellCompleter {}
