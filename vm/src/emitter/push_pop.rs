use super::names::*;
use crate::ast::*;

pub fn emit_push(segment: Segment, statics_base: &str, index: u16) -> Vec<String> {
    match segment {
        Segment::Argument => emit_indirect_push("@ARG", index),
        Segment::Local => emit_indirect_push("@LCL", index),
        Segment::Static => emit_direct_push(&make_ref(&make_static_name(statics_base, index)), 0),
        Segment::Constant => emit_constant_push(&index.to_string()),
        Segment::This => emit_indirect_push("@THIS", index),
        Segment::That => emit_indirect_push("@THAT", index),
        Segment::Pointer => emit_direct_push("@THIS", index),
        Segment::Temp => emit_direct_push("@R5", index),
    }
}

pub fn emit_indirect_push(segment: &str, index: u16) -> Vec<String> {
    let mut results = Vec::new();

    if index != 0 {
        results.push(make_ref(&index.to_string()));
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

pub fn emit_direct_push(reference: &str, index: u16) -> Vec<String> {
    let mut results = Vec::new();

    if index != 0 {
        results.push(make_ref(&index.to_string()));
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

pub fn emit_constant_push(value: &str) -> Vec<String> {
    let mut results = Vec::new();

    results.push(make_ref(value));
    results.push("D=A".to_string()); //    D=A
    results.append(&mut push_d());

    results
}

pub fn emit_pop(segment: Segment, statics_base: &str, index: u16) -> Vec<String> {
    match segment {
        Segment::Argument => emit_indirect_pop("@ARG", index),
        Segment::Local => emit_indirect_pop("@LCL", index),
        Segment::Static => emit_direct_pop(&make_ref(&make_static_name(statics_base, index)), 0),
        Segment::Constant => panic!("Can't pop to a constant"),
        Segment::This => emit_indirect_pop("@THIS", index),
        Segment::That => emit_indirect_pop("@THAT", index),
        Segment::Pointer => emit_direct_pop("@THIS", index),
        Segment::Temp => emit_direct_pop("@R5", index),
    }
}

pub fn emit_direct_pop(reference: &str, index: u16) -> Vec<String> {
    if index == 0 {
        let mut results = pop_d();
        results.push(reference.to_string());
        results.push("M=D".to_string());

        results
    } else {
        let mut results = Vec::new();
        results.push(reference.to_string());
        results.push("D=A".to_string());
        results.push(make_ref(&index.to_string()));
        results.push("D=D+A".to_string());
        results.push("@R13".to_string()); // spill the computed location to a temporary because
        results.push("M=D".to_string()); // we're about to need all the registers to pop into D
        results.append(&mut emit_indirect_pop("@R13", 0));
        results
    }
}

pub fn emit_indirect_pop(segment: &str, index: u16) -> Vec<String> {
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
        results.push(make_ref(&index.to_string()));
        results.push("D=D+A".to_string());
        results.push("@R13".to_string()); // spill the computed location to a temporary because
        results.push("M=D".to_string()); // we're about to need all the registers to pop into D
        results.append(&mut emit_indirect_pop("@R13", 0));
        results
    }
}

pub fn push_d() -> Vec<String> {
    let mut results = Vec::new();

    results.push("@SP".to_string()); //    A=&SP
    results.push("M=M+1".to_string()); //  SP=SP+1
    results.push("A=M-1".to_string()); //  A=SP-1
    results.push("M=D".to_string()); //    *A=D

    results
}

// note, clobbers a
// doesn't move the stack pointer and leave
// m pointing to the last value
pub fn peek_m() -> Vec<String> {
    let mut results = Vec::new();
    results.push("@SP".to_string()); //     A=&SP
    results.push("A=M-1".to_string()); //   A=SP-1
    results
}

// note, clobbers a and leave m pointing to the old top of stack
pub fn pop_m() -> Vec<String> {
    let mut results = Vec::new();
    results.push("@SP".to_string()); //     A=&SP
    results.push("M=M-1".to_string()); //   SP=SP-1
    results.push("A=M".to_string()); //   A=SP
    results
}

// note, clobbers a and leaves d=*sp
pub fn pop_d() -> Vec<String> {
    let mut results = pop_m();
    results.push("D=M".to_string());
    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_emit_push() {
        assert_eq!(
            emit_push(Segment::Argument, "foo", 2),
            vec!["@2", "D=A", "@ARG", "A=M", "A=A+D", "D=M", "@SP", "M=M+1", "A=M-1", "M=D"]
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
            vec![
                "@ARG", "D=M", "@2", "D=D+A", "@R13", "M=D", "@SP", "M=M-1", "A=M", "D=M", "@R13",
                "A=M", "M=D"
            ]
        );
    }
}
