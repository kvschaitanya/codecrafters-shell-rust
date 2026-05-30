pub struct ShellLexer<'a> {
    chars: std::str::Chars<'a>,
}

impl<'a> ShellLexer<'a> {
    fn double_quotes(&mut self, token: &mut String) {
        while let Some(c) = self.chars.next() {
            match c {
                '\\' => match self.chars.next() {
                    Some(ch @ ('"' | '\\')) => token.push(ch),
                    Some(other) => {
                        token.push('\\');
                        token.push(other)
                    }
                    None => token.push('\\'),
                },
                '"' => break,
                _ => token.push(c),
            }
        }
    }
}

impl<'a> Iterator for ShellLexer<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut token: String = String::new();

        while let Some(c) = self.chars.next() {
            match c {
                '\\' => {
                    if let Some(ch) = self.chars.next() {
                        token.push(ch);
                    }
                }
                '\'' => token.extend(self.chars.by_ref().take_while(|&ch| ch != '\'')),
                //'"' => token.extend(self.chars.by_ref().take_while(|&ch| ch != '"')),
                '"' => self.double_quotes(&mut token),
                ' ' if token.is_empty() => continue,
                ' ' => break,
                _ => token.push(c),
            }
        }
        if token.is_empty() { None } else { Some(token) }
    }
}

pub trait LexerExt {
    fn tokenize(&self) -> ShellLexer<'_>;
}

impl LexerExt for str {
    fn tokenize(&self) -> ShellLexer<'_> {
        ShellLexer {
            chars: self.chars(),
        }
    }
}
