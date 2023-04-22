use super::push_pop::*;

pub fn emit_binary(op: BinaryOp) -> Vec<String> {
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

pub fn emit_unary(op: UnaryOp) -> Vec<String> {
    let mut results = peek_m();

    results.push(match op {
        UnaryOp::Not => "M=!M".to_string(), // *sp = !*sp     (arg = !arg)
        UnaryOp::Neg => "M=-M".to_string(), // *sp = -*sp     (arg = -arg)
    });
    results
}

pub enum BinaryOp {
    Add,
    Sub,
    Or,
    And,
}

pub enum UnaryOp {
    Neg,
    Not,
}

#[cfg(test)]
mod test {
    use super::*;

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
}
