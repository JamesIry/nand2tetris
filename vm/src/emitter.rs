#![allow(clippy::vec_init_then_push)]

mod branching;
mod compare;
mod function;
mod math_logic;
mod names;
mod push_pop;

use branching::*;
use compare::*;
use function::*;
use math_logic::*;
use push_pop::*;

use crate::ast::*;
use crate::printer;

pub fn emit_commands(
    commands: Vec<Command>,
    statics_base: &str,
    bootstrap: bool,
) -> impl IntoIterator<Item = String> {
    let mut current_function = statics_base.to_string();
    let mut label_number = 0;

    let mut results = if bootstrap {
        emit_bootstrap(statics_base, &mut current_function, &mut label_number)
    } else {
        Vec::new()
    };

    results.extend(commands.into_iter().flat_map(|command| {
        emit_command(
            command,
            statics_base,
            &mut current_function,
            &mut label_number,
        )
    }));
    results
}

fn emit_bootstrap(
    statics_base: &str,
    current_function: &mut String,
    label_number: &mut usize,
) -> Vec<String> {
    let mut results = Vec::new();
    results.push("// bootstrap SP".to_string());
    results.push("@256".to_string());
    results.push("D=A".to_string());
    results.push("@SP".to_string());
    results.push("M=D".to_string());
    results.append(&mut emit_command(
        Command::Call("Sys.init".to_string(), 0),
        statics_base,
        current_function,
        label_number,
    ));
    results
}

fn emit_command(
    command: Command,
    statics_base: &str,
    current_function: &mut String,
    label_number: &mut usize,
) -> Vec<String> {
    let mut results = Vec::new();
    results.push(emit_comment(&command));
    results.append(&mut match command {
        Command::Push(segment, index) => emit_push(segment, statics_base, index),
        Command::Pop(segment, index) => emit_pop(segment, statics_base, index),
        Command::Add => emit_binary(BinaryOp::Add),
        Command::Sub => emit_binary(BinaryOp::Sub),
        Command::Neg => emit_unary(UnaryOp::Neg),
        Command::Eq => emit_compare(CompareOp::Eq, current_function, label_number),
        Command::Gt => emit_compare(CompareOp::Gt, current_function, label_number),
        Command::Lt => emit_compare(CompareOp::Lt, current_function, label_number),
        Command::And => emit_binary(BinaryOp::And),
        Command::Or => emit_binary(BinaryOp::Or),
        Command::Not => emit_unary(UnaryOp::Not),
        Command::Goto(label) => emit_goto(current_function, &label),
        Command::IfGoto(label) => emit_if_goto(current_function, &label),
        Command::Label(label) => emit_label(current_function, &label),
        Command::Function(function, n_locals) => {
            emit_function(&function, n_locals, current_function, label_number)
        }
        Command::Call(function, n_args) => emit_call(&function, n_args, label_number),
        Command::Return => emit_return(),
        Command::Comment(_) => Vec::new(),
    });
    results
}

fn emit_comment(command: &Command) -> String {
    let mut string = String::new();
    string.push_str("// ");
    string.push_str(&printer::print_command(command));
    string
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_emit_comment() {
        assert_eq!(emit_comment(&Command::Add), "// add");
    }

    #[test]
    fn test_emit_command() {
        assert_eq!(
            emit_command(Command::Add, "foo", &mut "bar".to_string(), &mut 123),
            vec!["// add", "@SP", "M=M-1", "A=M", "D=M", "A=A-1", "M=D+M"]
        );
    }
}
