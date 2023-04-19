use std::collections::HashSet;

use crate::ast::*;

pub fn parse_lines<T, S>(lines: T) -> (Vec<Instruction>, Vec<(usize, ParseError)>)
where
    T: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let results = lines
        .into_iter()
        .map(|line| parse_line(line.as_ref()))
        .enumerate()
        .filter_map(|(n, result)| result.map(|r| (n, r)));

    let mut instructions = Vec::new();
    let mut errors = Vec::new();
    for (line, result) in results {
        match result {
            Ok(instruction) => instructions.push(instruction),
            Err(error) => errors.push((line, error)),
        }
    }
    (instructions, errors)
}

fn parse_line(line: &str) -> Option<Result<Instruction, ParseError>> {
    let comment = line.find("//").unwrap_or(line.len());
    let stripped = &line[..comment];

    let trimmed = stripped.trim();

    if trimmed.is_empty() {
        None
    } else {
        Some(parse_instruction(trimmed))
    }
}

fn parse_instruction(instruction: &str) -> Result<Instruction, ParseError> {
    let trimmed = instruction.trim();

    if trimmed.starts_with('(') {
        parse_l_inst(trimmed)
    } else if trimmed.starts_with('@') {
        parse_a_inst(trimmed)
    } else {
        parse_c_inst(trimmed)
    }
}

fn parse_l_inst(instruction: &str) -> Result<Instruction, ParseError> {
    let tail = &instruction['('.len_utf8()..];
    if tail.is_empty() {
        Err(ParseError::MissingClosingParen(instruction.to_string()))
    } else {
        let last = tail.chars().last();
        if last != Some(')') {
            Err(ParseError::MissingClosingParen(instruction.to_string()))
        } else {
            let symbol = &tail[..tail.len() - ')'.len_utf8()];
            Ok(Instruction::L(symbol.trim().to_string()))
        }
    }
}

fn parse_a_inst(instruction: &str) -> Result<Instruction, ParseError> {
    let tail = &instruction['@'.len_utf8()..];
    let symbol = parse_reference(tail);
    symbol.map(Instruction::A)
}

fn parse_reference(symbol: &str) -> Result<Reference, ParseError> {
    let trimmed = symbol.trim();
    if trimmed.is_empty() {
        Err(ParseError::MissingSymbol)
    } else {
        let first_char = trimmed.chars().next().unwrap();
        if first_char.is_numeric() {
            let n = parse_constant(trimmed)?;
            Ok(Reference::Address(n))
        } else {
            parse_symbol(trimmed)
        }
    }
}

fn parse_constant(trimmed: &str) -> Result<u16, ParseError> {
    let parsed = trimmed.parse::<u16>();
    let n = parsed.map_err(|_| ParseError::InvalidSymbol(trimmed.to_string()))?;
    if n <= 32767 {
        Ok(n)
    } else {
        Err(ParseError::ConstantOutOfRange(n))
    }
}

fn parse_symbol(string: &str) -> Result<Reference, ParseError> {
    Ok(Reference::Symbol(string.to_string()))
}

fn parse_c_inst(instruction: &str) -> Result<Instruction, ParseError> {
    // dest = compute; jump
    // dest and jump are optional [dest = ] compute [;jump]
    let opt_dest_index = instruction.find('=');
    let (dest, compute_and_jump) = opt_dest_index
        .map(|dest_index| {
            (
                &instruction[..dest_index],
                &instruction[dest_index + '='.len_utf8()..],
            )
        })
        .unwrap_or(("", instruction));
    let dest = parse_dest(dest)?;

    let opt_jump_index = compute_and_jump.find(';');
    let (expr, jump) = opt_jump_index
        .map(|jump_index| {
            (
                &compute_and_jump[..jump_index],
                &compute_and_jump[jump_index + ';'.len_utf8()..],
            )
        })
        .unwrap_or((compute_and_jump, ""));
    let jump = parse_jump(jump)?;

    let expr = parse_expr(expr)?;

    Ok(Instruction::C(dest, expr, jump))
}

fn parse_expr(compute: &str) -> Result<Expr, ParseError> {
    let mut trimmed = compute.to_string();
    trimmed.retain(|ch| !ch.is_whitespace());
    match trimmed.as_str() {
        "0" => Ok(Expr::Zero),
        "1" => Ok(Expr::One),
        "-1" => Ok(Expr::NegOne),
        "D" => Ok(Expr::D),
        "A" => Ok(Expr::A),
        "M" => Ok(Expr::M),
        "!D" => Ok(Expr::NotD),
        "!A" => Ok(Expr::NotA),
        "!M" => Ok(Expr::NotM),
        "-D" => Ok(Expr::NegD),
        "-A" => Ok(Expr::NegA),
        "-M" => Ok(Expr::NegM),
        "D+1" => Ok(Expr::DAddOne),
        "A+1" => Ok(Expr::AAddOne),
        "M+1" => Ok(Expr::MAddOne),
        "D-1" => Ok(Expr::DSubOne),
        "A-1" => Ok(Expr::ASubOne),
        "M-1" => Ok(Expr::MSubOne),
        "D+A" => Ok(Expr::DAddA),
        "D+M" => Ok(Expr::DAddM),
        "D-A" => Ok(Expr::DSubA),
        "D-M" => Ok(Expr::DSubM),
        "A-D" => Ok(Expr::ASubD),
        "M-D" => Ok(Expr::MSubD),
        "D&A" => Ok(Expr::DAndA),
        "D&M" => Ok(Expr::DAndM),
        "D|A" => Ok(Expr::DOrA),
        "D|M" => Ok(Expr::DOrM),
        _ => Err(ParseError::InvalidExpr(compute.to_string())),
    }
}

fn parse_register(register: char) -> Result<Register, ParseError> {
    match register {
        'A' => Ok(Register::A),
        'D' => Ok(Register::D),
        'M' => Ok(Register::M),
        _ => Err(ParseError::InvalidRegister(register.to_string())),
    }
}

fn parse_dest(dest: &str) -> Result<HashSet<Register>, ParseError> {
    let mut trimmed = dest.to_string();
    trimmed.retain(|ch| !ch.is_whitespace());

    let mut registers = HashSet::with_capacity(3);
    for ch in trimmed.chars() {
        let register = parse_register(ch)?;
        if registers.contains(&register) {
            return Err(ParseError::DuplicateRegister(dest.to_string()));
        }
        registers.insert(register);
    }
    Ok(registers)
}

fn parse_jump(jump: &str) -> Result<Jump, ParseError> {
    let trimmed = jump.trim();
    match trimmed {
        "" => Ok(Jump::Null),
        "JGT" => Ok(Jump::Jgt),
        "JEQ" => Ok(Jump::Jeq),
        "JGE" => Ok(Jump::Jge),
        "JLT" => Ok(Jump::Jlt),
        "JNE" => Ok(Jump::Jne),
        "JLE" => Ok(Jump::Jle),
        "JMP" => Ok(Jump::Jmp),
        _ => Err(ParseError::InvalidJumpType(jump.to_string())),
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum ParseError {
    MissingSymbol,
    InvalidSymbol(String),
    MissingClosingParen(String),
    InvalidJumpType(String),
    ConstantOutOfRange(u16),
    InvalidRegister(String),
    InvalidExpr(String),
    DuplicateRegister(String),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_symbol() {
        assert_eq!(
            parse_symbol("hello"),
            Ok(Reference::Symbol("hello".to_string()))
        );
    }

    #[test]
    fn test_parse_consant() {
        assert_eq!(parse_constant("123"), Ok(123));
        assert_eq!(
            parse_constant("60000"),
            Err(ParseError::ConstantOutOfRange(60000))
        );
        assert_eq!(
            parse_constant("foo"),
            Err(ParseError::InvalidSymbol("foo".to_string()))
        );
    }

    #[test]
    fn test_parse_reference() {
        assert_eq!(parse_reference("123"), Ok(Reference::Address(123)));
        assert_eq!(
            parse_reference("hello"),
            Ok(Reference::Symbol("hello".to_string()))
        );
        assert_eq!(parse_reference(""), Err(ParseError::MissingSymbol));
    }

    #[test]
    fn test_parse_a_inst() {
        assert_eq!(
            parse_a_inst("@123"),
            Ok(Instruction::A(Reference::Address(123)))
        );
        assert_eq!(
            parse_a_inst("@hello"),
            Ok(Instruction::A(Reference::Symbol("hello".to_string())))
        );
        assert_eq!(parse_a_inst("@"), Err(ParseError::MissingSymbol));
    }

    #[test]
    fn test_parse_l_inst() {
        assert_eq!(parse_l_inst("(abc)"), Ok(Instruction::L("abc".to_string())));
        assert_eq!(
            parse_l_inst("(abc"),
            Err(ParseError::MissingClosingParen("(abc".to_string()))
        );
        assert_eq!(
            parse_l_inst("("),
            Err(ParseError::MissingClosingParen("(".to_string()))
        );
    }

    #[test]
    fn test_parse_dest() {
        assert_eq!(parse_dest(""), Ok(HashSet::new()));
        assert_eq!(parse_dest("A"), Ok(HashSet::from([Register::A])));
        assert_eq!(parse_dest("D"), Ok(HashSet::from([Register::D])));
        assert_eq!(parse_dest("M"), Ok(HashSet::from([Register::M])));
        assert_eq!(
            parse_dest(" A M D "),
            Ok(HashSet::from([Register::A, Register::D, Register::M]))
        );
        assert_eq!(
            parse_dest("AMMD"),
            Err(ParseError::DuplicateRegister("AMMD".to_string()))
        );
        assert_eq!(
            parse_dest("QBS"),
            Err(ParseError::InvalidRegister("Q".to_string()))
        );
    }

    #[test]
    fn test_parse_jump() {
        assert_eq!(parse_jump(""), Ok(Jump::Null));
        assert_eq!(parse_jump("JGT"), Ok(Jump::Jgt));
        assert_eq!(parse_jump("JEQ"), Ok(Jump::Jeq));
        assert_eq!(parse_jump("JGE"), Ok(Jump::Jge));
        assert_eq!(parse_jump("JLT"), Ok(Jump::Jlt));
        assert_eq!(parse_jump("JNE"), Ok(Jump::Jne));
        assert_eq!(parse_jump("JLE"), Ok(Jump::Jle));
        assert_eq!(parse_jump("JMP"), Ok(Jump::Jmp));
        assert_eq!(
            parse_jump("JUK"),
            Err(ParseError::InvalidJumpType("JUK".to_string()))
        );
    }

    #[test]
    fn test_parse_register() {
        assert_eq!(parse_register('A'), Ok(Register::A));
        assert_eq!(parse_register('D'), Ok(Register::D));
        assert_eq!(parse_register('M'), Ok(Register::M));
        assert_eq!(
            parse_register('Q'),
            Err(ParseError::InvalidRegister("Q".to_string()))
        );
    }

    #[test]
    fn test_parse_expr() {
        assert_eq!(parse_expr("0"), Ok(Expr::Zero));
        assert_eq!(parse_expr("1"), Ok(Expr::One));
        assert_eq!(parse_expr("-1"), Ok(Expr::NegOne));
        assert_eq!(parse_expr("D"), Ok(Expr::D));
        assert_eq!(parse_expr("A"), Ok(Expr::A));
        assert_eq!(parse_expr("M"), Ok(Expr::M));
        assert_eq!(parse_expr("!D"), Ok(Expr::NotD));
        assert_eq!(parse_expr("!A"), Ok(Expr::NotA));
        assert_eq!(parse_expr("!M"), Ok(Expr::NotM));
        assert_eq!(parse_expr("-D"), Ok(Expr::NegD));
        assert_eq!(parse_expr("-A"), Ok(Expr::NegA));
        assert_eq!(parse_expr("-M"), Ok(Expr::NegM));
        assert_eq!(parse_expr("D + 1"), Ok(Expr::DAddOne));
        assert_eq!(parse_expr("A+1"), Ok(Expr::AAddOne));
        assert_eq!(parse_expr(" M+1"), Ok(Expr::MAddOne));
        assert_eq!(parse_expr("D-1 "), Ok(Expr::DSubOne));
        assert_eq!(parse_expr("A-1"), Ok(Expr::ASubOne));
        assert_eq!(parse_expr("M- 1"), Ok(Expr::MSubOne));
        assert_eq!(parse_expr("D+A"), Ok(Expr::DAddA));
        assert_eq!(parse_expr("D + M"), Ok(Expr::DAddM));
        assert_eq!(parse_expr("D-A"), Ok(Expr::DSubA));
        assert_eq!(parse_expr("D-M"), Ok(Expr::DSubM));
        assert_eq!(parse_expr("A-D"), Ok(Expr::ASubD));
        assert_eq!(parse_expr("M-D"), Ok(Expr::MSubD));
        assert_eq!(parse_expr("D&A"), Ok(Expr::DAndA));
        assert_eq!(parse_expr("D&M"), Ok(Expr::DAndM));
        assert_eq!(parse_expr("D|A"), Ok(Expr::DOrA));
        assert_eq!(parse_expr("D| M"), Ok(Expr::DOrM));
        assert_eq!(
            parse_expr("M | D"),
            Err(ParseError::InvalidExpr("M | D".to_string()))
        );
    }

    #[test]
    fn test_parse_c_inst() {
        assert_eq!(
            parse_c_inst(" ADM = D + 1; JLT"),
            Ok(Instruction::C(
                HashSet::from([Register::A, Register::D, Register::M]),
                Expr::DAddOne,
                Jump::Jlt
            ))
        );

        assert_eq!(
            parse_c_inst(" D + 1; JLT"),
            Ok(Instruction::C(HashSet::new(), Expr::DAddOne, Jump::Jlt))
        );

        assert_eq!(
            parse_c_inst(" ADM = D + 1"),
            Ok(Instruction::C(
                HashSet::from([Register::A, Register::D, Register::M]),
                Expr::DAddOne,
                Jump::Null
            ))
        );
    }

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            parse_instruction("(abc)"),
            Ok(Instruction::L("abc".to_string()))
        );

        assert_eq!(
            parse_instruction(" ADM = D + 1; JLT"),
            Ok(Instruction::C(
                HashSet::from([Register::A, Register::D, Register::M]),
                Expr::DAddOne,
                Jump::Jlt
            ))
        );

        assert_eq!(
            parse_instruction("@123"),
            Ok(Instruction::A(Reference::Address(123)))
        );
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("  (abc) "),
            Some(Ok(Instruction::L("abc".to_string())))
        );

        assert_eq!(
            parse_line("  (abc) // blah blah"),
            Some(Ok(Instruction::L("abc".to_string())))
        );

        assert_eq!(parse_line("   "), None);
        assert_eq!(parse_line("  // flub"), None);
    }

    #[test]
    fn test_parse_lines() {
        let results = parse_lines(vec!["  (abc) ", "   ", "  // flub", "dog", "D+M", "cat"]);

        assert_eq!(
            results.0,
            vec![
                Instruction::L("abc".to_string()),
                Instruction::C(HashSet::new(), Expr::DAddM, Jump::Null)
            ]
        );
        assert_eq!(
            results.1,
            vec![
                (3, ParseError::InvalidExpr("dog".to_string())),
                (5, ParseError::InvalidExpr("cat".to_string()))
            ]
        );
    }
}
