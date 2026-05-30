pub enum OutputTarget {
    Stdout,
    File(String),
    AppendFile(String),
}

pub enum ErrorTarget {
    Stderr,
    File(String),
    AppendFile(String),
}

pub struct ShellCommand {
    pub command: String,
    pub args: Vec<String>,
    pub output: OutputTarget,
    pub error: ErrorTarget,
}

pub fn parse_commands(mut tokens: impl Iterator<Item = String>) -> Vec<ShellCommand> {
    let mut commands = vec![];

    while let Some(command) = tokens.next() {
        let mut args: Vec<String> = vec![];
        let mut output = OutputTarget::Stdout;
        let mut error = ErrorTarget::Stderr;

        while let Some(token) = tokens.next() {
            match token.as_str() {
                ">" | "1>" => {
                    if let Some(file) = tokens.next() {
                        output = OutputTarget::File(file);
                    }
                }
                "2>" => {
                    if let Some(file) = tokens.next() {
                        error = ErrorTarget::File(file);
                    }
                }
                ">>" | "1>>" => {
                    if let Some(file) = tokens.next() {
                        output = OutputTarget::AppendFile(file);
                    }
                }
                "2>>" => {
                    if let Some(file) = tokens.next() {
                        error = ErrorTarget::AppendFile(file);
                    }
                }
                _ => args.push(token),
            };
        }

        commands.push(ShellCommand {
            command,
            args,
            output,
            error,
        });
    }
    commands
}
