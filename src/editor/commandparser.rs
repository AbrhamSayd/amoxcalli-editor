#[derive(Debug, PartialEq)]
pub enum ParsedCommand {
    Write,              // :w
    Quit,               // :q
    WriteQuit,          // :wq or :x
    ForceQuit,          // :q!
    WriteAsAndQuit(String), // :wq filename
    WriteAs(String),    // :w filename
    Unknown(String),
}

impl ParsedCommand {
    pub fn parse(input: &str) -> Self {
        let trimmed = input.trim();
        
        if trimmed.is_empty() {
            return Self::Unknown(String::new());
        }
        
        // Split the command and arguments
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        let command = parts[0];
        let args = &parts[1..];
        
        match command {
            "w" | "write" => {
                if args.is_empty() {
                    Self::Write
                } else {
                    Self::WriteAs(args.join(" "))
                }
            }
            "q" | "quit" => Self::Quit,
            "q!" | "quit!" => Self::ForceQuit,
            "wq" | "x" => {
                if args.is_empty() {
                    Self::WriteQuit
                } else {
                    Self::WriteAsAndQuit(args.join(" "))
                }
            }
            _ => Self::Unknown(trimmed.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_commands() {
        assert_eq!(ParsedCommand::parse("w"), ParsedCommand::Write);
        assert_eq!(ParsedCommand::parse("write"), ParsedCommand::Write);
        assert_eq!(ParsedCommand::parse("q"), ParsedCommand::Quit);
        assert_eq!(ParsedCommand::parse("quit"), ParsedCommand::Quit);
        assert_eq!(ParsedCommand::parse("q!"), ParsedCommand::ForceQuit);
        assert_eq!(ParsedCommand::parse("wq"), ParsedCommand::WriteQuit);
        assert_eq!(ParsedCommand::parse("x"), ParsedCommand::WriteQuit);
        assert_eq!(
            ParsedCommand::parse("w test.txt"),
            ParsedCommand::WriteAs("test.txt".to_string())
        );
        assert_eq!(
            ParsedCommand::parse("wq test.txt"),
            ParsedCommand::WriteAsAndQuit("test.txt".to_string())
        );
    }
}