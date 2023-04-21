use crate::ast::*;

pub fn parse_lines<S, T>(lines: T) -> Result<Vec<Command>, Vec<(usize, ParseError)>>
where
    T: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let results = lines
        .into_iter()
        .map(|line| parse_line(line.as_ref()))
        .enumerate()
        .filter_map(|(n, result)| result.map(|r| (n, r)));
    let mut commands = Vec::new();
    let mut errors = Vec::new();
    for (line, result) in results {
        match result {
            Ok(command) => commands.push(command),
            Err(error) => errors.push((line, error)),
        }
    }

    if errors.is_empty() {
        Ok(commands)
    } else {
        Err(errors)
    }
}

fn parse_line(line: &str) -> Option<Result<Command, ParseError>> {
    let parts: Vec<_> = line.split("//").collect();
    if parts.is_empty() {
        None
    } else {
        let trimmed = parts[0].trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(parse_command(trimmed))
        }
    }
}

fn parse_command(command: &str) -> Result<Command, ParseError> {
    let words: Vec<_> = command.split_whitespace().collect();
    match words.len() {
        1 => parse_command1(words[0]),
        3 => parse_command3(words[0], words[1], words[2]),
        _ => Err(ParseError::InvalidCommand(words[0].to_string())),
    }
}

fn parse_command1(command: &str) -> Result<Command, ParseError> {
    match command {
        "add" => Ok(Command::Add),
        "sub" => Ok(Command::Sub),
        "neg" => Ok(Command::Neg),
        "eq" => Ok(Command::Eq),
        "gt" => Ok(Command::Gt),
        "lt" => Ok(Command::Lt),
        "and" => Ok(Command::And),
        "or" => Ok(Command::Or),
        "not" => Ok(Command::Not),
        _ => Err(ParseError::Invalid1WordCommand(command.to_string())),
    }
}

fn parse_command3(command: &str, arg1: &str, arg2: &str) -> Result<Command, ParseError> {
    match command {
        "push" => {
            let segment = parse_segment(arg1)?;
            let index = parse_index(arg2)?;
            Ok(Command::Push(segment, index))
        }
        "pop" => {
            let segment = parse_segment(arg1)?;
            let index = parse_index(arg2)?;
            Ok(Command::Pop(segment, index))
        }
        _ => Err(ParseError::Invalid3WordCommand(command.to_string())),
    }
}

fn parse_segment(arg1: &str) -> Result<Segment, ParseError> {
    match arg1 {
        "argument" => Ok(Segment::Argument),
        "local" => Ok(Segment::Local),
        "static" => Ok(Segment::Static),
        "constant" => Ok(Segment::Constant),
        "this" => Ok(Segment::This),
        "that" => Ok(Segment::That),
        "pointer" => Ok(Segment::Pointer),
        "temp" => Ok(Segment::Temp),
        _ => Err(ParseError::InvalidSegment(arg1.to_string())),
    }
}

fn parse_index(arg2: &str) -> Result<u16, ParseError> {
    let parsed = arg2.parse::<u16>();
    let n = parsed.map_err(|_| ParseError::InvalidIndex(arg2.to_string()))?;
    if n <= 32767 {
        Ok(n)
    } else {
        Err(ParseError::IndexOutOfRange(n))
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ParseError {
    InvalidCommand(String),
    Invalid1WordCommand(String),
    Invalid3WordCommand(String),
    InvalidSegment(String),
    InvalidIndex(String),
    IndexOutOfRange(u16),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_index() {
        assert_eq!(parse_index("123"), Ok(123));
        assert_eq!(
            parse_index("60000"),
            Err(ParseError::IndexOutOfRange(60000))
        );
        assert_eq!(
            parse_index("foo"),
            Err(ParseError::InvalidIndex("foo".to_string()))
        );
    }

    #[test]
    fn test_parse_segment() {
        assert_eq!(parse_segment("argument"), Ok(Segment::Argument));
        assert_eq!(parse_segment("local"), Ok(Segment::Local));
        assert_eq!(parse_segment("static"), Ok(Segment::Static));
        assert_eq!(parse_segment("constant"), Ok(Segment::Constant));
        assert_eq!(parse_segment("this"), Ok(Segment::This));
        assert_eq!(parse_segment("that"), Ok(Segment::That));
        assert_eq!(parse_segment("pointer"), Ok(Segment::Pointer));
        assert_eq!(parse_segment("temp"), Ok(Segment::Temp));
        assert_eq!(
            parse_segment("foo"),
            Err(ParseError::InvalidSegment("foo".to_string()))
        );
    }

    #[test]
    fn test_parse_command3() {
        assert_eq!(
            parse_command3("push", "argument", "1"),
            Ok(Command::Push(Segment::Argument, 1))
        );
        assert_eq!(
            parse_command3("push", "argument", "1"),
            Ok(Command::Push(Segment::Argument, 1))
        );
        assert_eq!(
            parse_command3("foo", "whatever", "yeah"),
            Err(ParseError::Invalid3WordCommand("foo".to_string()))
        );
    }

    #[test]
    fn test_parse_command1() {
        assert_eq!(parse_command1("add"), Ok(Command::Add));
        assert_eq!(parse_command1("sub"), Ok(Command::Sub));
        assert_eq!(parse_command1("neg"), Ok(Command::Neg));
        assert_eq!(parse_command1("eq"), Ok(Command::Eq));
        assert_eq!(parse_command1("gt"), Ok(Command::Gt));
        assert_eq!(parse_command1("lt"), Ok(Command::Lt));
        assert_eq!(parse_command1("and"), Ok(Command::And));
        assert_eq!(parse_command1("or"), Ok(Command::Or));
        assert_eq!(parse_command1("not"), Ok(Command::Not));
        assert_eq!(
            parse_command1("foo"),
            Err(ParseError::Invalid1WordCommand("foo".to_string()))
        );
    }

    #[test]
    fn test_parse_command() {
        assert_eq!(parse_command("add"), Ok(Command::Add));
        assert_eq!(
            parse_command("push argument 1"),
            Ok(Command::Push(Segment::Argument, 1))
        );
        assert_eq!(
            parse_command("foo whatever yeah"),
            Err(ParseError::Invalid3WordCommand("foo".to_string()))
        );
        assert_eq!(
            parse_command("foo whatever yeah baby"),
            Err(ParseError::InvalidCommand("foo".to_string()))
        );
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line(" add // ok"), Some(Ok(Command::Add)));
        assert_eq!(
            parse_line(" push  argument  1   // comment"),
            Some(Ok(Command::Push(Segment::Argument, 1)))
        );
        assert_eq!(parse_line("// comment"), None);
        assert_eq!(parse_line(""), None);
    }
}
