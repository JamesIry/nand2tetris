#![allow(clippy::vec_init_then_push)]

use crate::ast::*;
use crate::printer;

pub fn emit_commands<T>(commands: T, base_name: &str) -> impl IntoIterator<Item = String>
where
    T: IntoIterator<Item = Command>,
{
    let mut results = Vec::new();
    let mut label = 0;
    results.extend(
        commands
            .into_iter()
            .flat_map(|command| emit_command(command, base_name, &mut label)),
    );
    results
}
fn emit_command(command: Command, base_name: &str, label: &mut usize) -> Vec<String> {
    let mut results = vec![emit_comment(&command)];
    results.append(&mut match command {
        Command::Push(segment, index) => emit_push(segment, base_name, index),
        Command::Pop(segment, index) => emit_pop(segment, base_name, index),
        Command::Add => emit_binary(BinaryOp::Add),
        Command::Sub => emit_binary(BinaryOp::Sub),
        Command::Neg => emit_unary(UnaryOp::Neg),
        Command::Eq => emit_compare(CompareOp::Eq, base_name, label),
        Command::Gt => emit_compare(CompareOp::Gt, base_name, label),
        Command::Lt => emit_compare(CompareOp::Lt, base_name, label),
        Command::And => emit_binary(BinaryOp::And),
        Command::Or => emit_binary(BinaryOp::Or),
        Command::Not => emit_unary(UnaryOp::Not),
    });
    results
}

fn emit_push(segment: Segment, base_name: &str, index: u16) -> Vec<String> {
    match segment {
        Segment::Argument => emit_indirect_push("@ARG", index),
        Segment::Local => emit_indirect_push("@LCL", index),
        Segment::Static => emit_direct_push(&make_static_ref(base_name, index), 0),
        Segment::Constant => emit_constant_push(index),
        Segment::This => emit_indirect_push("@THIS", index),
        Segment::That => emit_indirect_push("@THAT", index),
        Segment::Pointer => emit_direct_push("@THIS", index),
        Segment::Temp => emit_direct_push("@R5", index),
    }
}

fn emit_indirect_push(segment: &str, index: u16) -> Vec<String> {
    let mut results = Vec::new();

    if index != 0 {
        results.push(make_simple_ref(&index.to_string()));
        results.push("D=A".to_string());
    }
    results.push(segment.to_string());
    results.push("A=M".to_string());
    if index != 0 {
        results.push("A=A+D".to_string());
    }
    results.push("D=M".to_string());
    results.append(&mut push_d());

    results
}

fn emit_direct_push(reference: &str, index: u16) -> Vec<String> {
    let mut results = Vec::new();

    if index != 0 {
        results.push(make_simple_ref(&index.to_string()));
        results.push("D=A".to_string());
    }
    results.push(reference.to_string());
    if index != 0 {
        results.push("A=A+D".to_string());
    }
    results.push("D=M".to_string());
    results.append(&mut push_d());
    results
}

fn emit_constant_push(index: u16) -> Vec<String> {
    let mut results = Vec::new();

    results.push(make_simple_ref(&index.to_string()));
    results.push("D=A".to_string()); //    D=A
    results.append(&mut push_d());

    results
}

fn push_d() -> Vec<String> {
    let mut results = Vec::new();

    results.push("@SP".to_string()); //    A=&SP
    results.push("M=M+1".to_string()); //  SP=SP+1
    results.push("A=M-1".to_string()); //  A=SP-1
    results.push("M=D".to_string()); //    *A=D

    results
}

fn emit_pop(segment: Segment, base_name: &str, index: u16) -> Vec<String> {
    match segment {
        Segment::Argument => emit_indirect_pop("@ARG", index),
        Segment::Local => emit_indirect_pop("@LCL", index),
        Segment::Static => emit_direct_pop(&make_static_ref(base_name, index), 0),
        Segment::Constant => panic!("Can't pop to a constant"),
        Segment::This => emit_indirect_pop("@THIS", index),
        Segment::That => emit_indirect_pop("@THAT", index),
        Segment::Pointer => emit_direct_pop("@THIS", index),
        Segment::Temp => emit_direct_pop("@R5", index),
    }
}

fn emit_direct_pop(reference: &str, index: u16) -> Vec<String> {
    if index == 0 {
        let mut results = pop_d();
        results.push(reference.to_string());
        results.push("M=D".to_string());

        results
    } else {
        let mut results = Vec::new();
        results.push(reference.to_string());
        results.push("D=A".to_string());
        results.push(make_simple_ref(&index.to_string()));
        results.push("D=D+A".to_string());
        results.push("@R13".to_string()); // spill the computed location to a temporary because
        results.push("M=D".to_string()); // we're about to need all the registers to pop into D
        results.append(&mut emit_indirect_pop("@R13", 0));
        results
    }
}

fn emit_indirect_pop(segment: &str, index: u16) -> Vec<String> {
    if index == 0 {
        let mut results = pop_d();
        results.push(segment.to_string());
        results.push("A=M".to_string());

        results.push("M=D".to_string());
        results
    } else {
        let mut results = Vec::new();
        results.push(segment.to_string());
        results.push("D=M".to_string());
        results.push(make_simple_ref(&index.to_string()));
        results.push("D=D+A".to_string());
        results.push("@R13".to_string()); // spill the computed location to a temporary because
        results.push("M=D".to_string()); // we're about to need all the registers to pop into D
        results.append(&mut emit_indirect_pop("@R13", 0));
        results
    }
}

fn make_simple_ref(name: &str) -> String {
    let mut string = "@".to_string();
    string.push_str(name);
    string
}

fn make_static_ref(base_name: &str, index: u16) -> String {
    let mut string = make_simple_ref(base_name);
    string.push('.');
    string.push_str(&index.to_string());
    string
}

fn make_labeled_ref(base_name: &str, label: &str, number: usize) -> String {
    let mut string = make_simple_ref(base_name);
    string.push('.');
    string.push_str(label);
    string.push('.');
    string.push_str(&number.to_string());
    string
}

fn emit_compare(op: CompareOp, base_name: &str, label: &mut usize) -> Vec<String> {
    let mut results = pop_d();

    results.push("A=A-1".to_string()); //  sp = sp-1       (sp = &arg1)
    results.push("D=M-D".to_string()); //  D = *sp - D   (D = arg1 - arg2)

    // branch to set D=result of comparison
    results.push(make_labeled_ref(base_name, "true", *label));

    // based on d, jump to @true or follow through
    results.push(compare_op(op));
    results.push("D=0".to_string()); // result is false
    results.push(make_labeled_ref(base_name, "join", *label));
    results.push("0;JMP".to_string()); // jump to join point
    results.push(make_label(base_name, "true", *label));
    results.push("D=-1".to_string()); // result is true
    results.push(make_label(base_name, "join", *label));

    results.append(&mut load_sp());

    // record the result
    results.push("M=D".to_string()); // *sp = result (arg1=result)

    *label += 1;

    results
}

fn compare_op(op: CompareOp) -> String {
    match op {
        CompareOp::Eq => "D;JEQ".to_string(),
        CompareOp::Gt => "D;JGT".to_string(),
        CompareOp::Lt => "D;JLT".to_string(),
    }
}

fn make_label(base_name: &str, label: &str, number: usize) -> String {
    let mut string = "(".to_string();
    string.push_str(base_name);
    string.push('.');
    string.push_str(label);
    string.push('.');
    string.push_str(&number.to_string());
    string.push(')');
    string
}

fn emit_binary(op: BinaryOp) -> Vec<String> {
    let mut results = pop_d();
    results.push("A=A-1".to_string());

    results.push(match op {
        BinaryOp::Sub => "M=M-D".to_string(), // *sp = *sp - D   (arg1 = arg1 - arg2)
        BinaryOp::Add => "M=D+M".to_string(), // *sp = D + *ap   (arg1 = arg2 + arg1)
        BinaryOp::And => "M=D&M".to_string(), // *sp = D & *ap   (arg1 = arg2 & arg1)
        BinaryOp::Or => "M=D|M".to_string(),  // *sp = D | *ap   (arg1 = arg2 | arg1)
    });

    results
}

fn emit_unary(op: UnaryOp) -> Vec<String> {
    let mut results = load_sp();

    results.push(match op {
        UnaryOp::Not => "M=!M".to_string(), // *sp = !*sp     (arg = !arg)
        UnaryOp::Neg => "M=-M".to_string(), // *sp = -*sp     (arg = -arg)
    });
    results
}

fn emit_comment(command: &Command) -> String {
    let mut string = String::new();
    string.push_str("// ");
    string.push_str(&printer::print_command(command));
    string
}

// note, clobbers a
// doesn't move the stack pointer and leave
// m pointing to the last value
fn load_sp() -> Vec<String> {
    let mut results = Vec::new();
    results.push("@SP".to_string()); //     A=&SP
    results.push("A=M-1".to_string()); //   A=SP-1
    results
}

// note, clobbers a and leave m pointing to the old top of stack
fn pop_m() -> Vec<String> {
    let mut results = Vec::new();
    results.push("@SP".to_string()); //     A=&SP
    results.push("M=M-1".to_string()); //   SP=SP-1
    results.push("A=M".to_string()); //   A=SP
    results
}

// note, clobbers a and leaves d=*sp
fn pop_d() -> Vec<String> {
    let mut results = pop_m();
    results.push("D=M".to_string());
    results
}
enum BinaryOp {
    Add,
    Sub,
    Or,
    And,
}

enum UnaryOp {
    Neg,
    Not,
}

enum CompareOp {
    Eq,
    Gt,
    Lt,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_emit_comment() {
        assert_eq!(emit_comment(&Command::Add), "// add");
    }

    #[test]
    fn test_emit_unary() {
        assert_eq!(emit_unary(UnaryOp::Not), vec!("@SP", "A=M-1", "M=!M"));
        assert_eq!(emit_unary(UnaryOp::Neg), vec!("@SP", "A=M-1", "M=-M"));
    }

    #[test]
    fn test_emit_binary() {
        assert_eq!(
            emit_binary(BinaryOp::Add),
            vec!["@SP", "M=M-1", "A=M", "D=M", "A=A-1", "M=D+M"]
        );
        assert_eq!(
            emit_binary(BinaryOp::Sub),
            vec!["@SP", "M=M-1", "A=M", "D=M", "A=A-1", "M=M-D"]
        );
        assert_eq!(
            emit_binary(BinaryOp::And),
            vec!["@SP", "M=M-1", "A=M", "D=M", "A=A-1", "M=D&M"]
        );
        assert_eq!(
            emit_binary(BinaryOp::Or),
            vec!["@SP", "M=M-1", "A=M", "D=M", "A=A-1", "M=D|M"]
        );
    }

    #[test]
    fn test_compare_op() {
        assert_eq!(compare_op(CompareOp::Eq), "D;JEQ");
        assert_eq!(compare_op(CompareOp::Gt), "D;JGT");
        assert_eq!(compare_op(CompareOp::Lt), "D;JLT");
    }

    #[test]
    fn test_emit_compare() {
        let mut label = 42;
        assert_eq!(
            emit_compare(CompareOp::Gt, "foo", &mut label),
            vec![
                "@SP",
                "M=M-1",
                "A=M",
                "D=M",
                "A=A-1",
                "D=M-D",
                "@foo.true.42",
                "D;JGT",
                "D=0",
                "@foo.join.42",
                "0;JMP",
                "(foo.true.42)",
                "D=-1",
                "(foo.join.42)",
                "@SP",
                "A=M-1",
                "M=D"
            ]
        );
        assert_eq!(label, 43);
    }

    #[test]
    fn test_emit_command() {
        let mut label = 123;
        assert_eq!(
            emit_command(Command::Add, "foo", &mut label),
            vec!["// add", "@SP", "M=M-1", "A=M", "D=M", "A=A-1", "M=D+M"]
        );
    }

    #[test]
    fn test_emit_push() {
        assert_eq!(
            emit_push(Segment::Argument, "foo", 2),
            vec!["@4", "D=M", "@SP", "M=M+1", "A=M-1", "M=D"]
        );
        assert_eq!(
            emit_push(Segment::Constant, "foo", 2),
            vec!["@2", "D=A", "@SP", "M=M+1", "A=M-1", "M=D"]
        );
    }

    #[test]
    fn test_emit_pop() {
        assert_eq!(
            emit_pop(Segment::Argument, "foo", 2),
            vec!["@SP", "M=M-1", "A=M", "D=M", "@4", "M=D"]
        );
    }
}
