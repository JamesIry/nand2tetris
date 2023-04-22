use super::names::*;
use super::push_pop::*;

pub fn emit_compare(op: CompareOp, function: &str, label_number: &mut usize) -> Vec<String> {
    let mut results = pop_d();
    let true_name = make_numbered_label_name(function, "true", *label_number);
    let join_name = make_numbered_label_name(function, "join", *label_number);

    results.push("A=A-1".to_string()); //  sp = sp-1       (sp = &arg1)
    results.push("D=M-D".to_string()); //  D = *sp - D   (D = arg1 - arg2)

    // branch to set D=result of comparison
    results.push(make_ref(&true_name));

    // based on d, jump to @true or follow through
    results.push(compare_op(op));
    results.push("D=0".to_string()); // result is false
    results.push(make_ref(&join_name));
    results.push("0;JMP".to_string()); // jump to join point
    results.push(make_label(&true_name));
    results.push("D=-1".to_string()); // result is true
    results.push(make_label(&join_name));

    results.append(&mut peek_m());

    // record the result
    results.push("M=D".to_string()); // *sp = result (arg1=result)

    *label_number += 1;

    results
}

fn compare_op(op: CompareOp) -> String {
    match op {
        CompareOp::Eq => "D;JEQ".to_string(),
        CompareOp::Gt => "D;JGT".to_string(),
        CompareOp::Lt => "D;JLT".to_string(),
    }
}
pub enum CompareOp {
    Eq,
    Gt,
    Lt,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compare_op() {
        assert_eq!(compare_op(CompareOp::Eq), "D;JEQ");
        assert_eq!(compare_op(CompareOp::Gt), "D;JGT");
        assert_eq!(compare_op(CompareOp::Lt), "D;JLT");
    }

    #[test]
    fn test_emit_compare() {
        let mut label_number = 42;
        assert_eq!(
            emit_compare(CompareOp::Gt, "bar", &mut label_number),
            vec![
                "@SP",
                "M=M-1",
                "A=M",
                "D=M",
                "A=A-1",
                "D=M-D",
                "@bar$true.42",
                "D;JGT",
                "D=0",
                "@bar$join.42",
                "0;JMP",
                "(bar$true.42)",
                "D=-1",
                "(bar$join.42)",
                "@SP",
                "A=M-1",
                "M=D"
            ]
        );
        assert_eq!(label_number, 43);
    }
}
