use super::names::*;
use super::push_pop::*;

pub fn emit_function(
    function: &str,
    n_locals: u16,
    current_function: &mut String,
    label_number: &mut usize,
) -> Vec<String> {
    *label_number = 0;
    current_function.clear();
    current_function.push_str(function);

    let mut results = Vec::new();
    results.push(make_label(function));
    for _ in 0..n_locals {
        results.append(&mut emit_constant_push("0"));
    }
    results
}

pub fn emit_call(function: &str, n_args: u16, label_number: &mut usize) -> Vec<String> {
    let return_name = make_numbered_label_name(function, "ret", *label_number);
    let mut results = Vec::new();

    // args start at SP-n_args
    // new_arg(R13) = SP-n_args
    results.push("@SP".to_string());
    results.push("D=M".to_string());
    results.push(make_ref(&n_args.to_string()));
    results.push("D=D-A".to_string());
    results.push("@13".to_string());
    results.push("M=D".to_string());

    // push the frame
    results.append(&mut emit_constant_push(&return_name));
    results.append(&mut emit_direct_push("@LCL", 0));
    results.append(&mut emit_direct_push("@ARG", 0));
    results.append(&mut emit_direct_push("@THIS", 0));
    results.append(&mut emit_direct_push("@THAT", 0));

    // ARG=new_arg(R13)
    results.push("@R13".to_string());
    results.push("D=M".to_string());
    results.push("@ARG".to_string());
    results.push("M=D".to_string());

    // set LCL to SP,
    results.push("@SP".to_string());
    results.push("D=M".to_string());
    results.push("@LCL".to_string());
    results.push("M=D".to_string());

    results.push(make_ref(function));
    results.push("0;JMP".to_string());
    results.push(make_label(&return_name));
    *label_number += 1;

    results
}

pub fn emit_return() -> Vec<String> {
    let mut results = Vec::new();

    // frame(R13)=LCL-5
    results.push("@5".to_string());
    results.push("D=A".to_string());
    results.push("@LCL".to_string());
    results.push("D=M-D".to_string());
    results.push("@R13".to_string());
    results.push("M=D".to_string());

    // retAddr(R14)=*frame
    results.push("A=D".to_string());
    results.push("D=M".to_string());
    results.push("@R14".to_string());
    results.push("M=D".to_string());

    // *ARG = pop()
    // i.e. put the return value in the first arg
    results.append(&mut emit_indirect_pop("@ARG", 0));

    // SP = ARG+1
    results.push("@ARG".to_string());
    results.push("D=M+1".to_string());
    results.push("@SP".to_string());
    results.push("M=D".to_string());

    // restore all the virtual registers
    results.append(&mut restore_register("@LCL"));
    results.append(&mut restore_register("@ARG"));
    results.append(&mut restore_register("@THIS"));
    results.append(&mut restore_register("@THAT"));

    // jump to the return address
    results.push("@R14".to_string());
    results.push("A=M".to_string());
    results.push("0;JMP".to_string());

    results
}

fn restore_register(register: &str) -> Vec<String> {
    let mut results = Vec::new();
    // frame=frame+1
    results.push("@R13".to_string());
    results.push("M=M+1".to_string());
    // restore virtual register
    results.push("A=M".to_string());
    results.push("D=M".to_string());
    results.push(register.to_string());
    results.push("M=D".to_string());

    results
}
