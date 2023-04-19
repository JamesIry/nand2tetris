use std::collections::{HashMap, HashSet};

use crate::ast::{Expr, Instruction, Jump, Reference, Register};

pub fn emit_instructions<'a, T>(instructions: T, symbol_table: &HashMap<String, u16>) -> Vec<u16>
where
    T: IntoIterator<Item = &'a Instruction>,
{
    instructions
        .into_iter()
        .flat_map(|instruction| emit_instruction(instruction, symbol_table))
        .collect()
}

pub fn emit_instruction(
    instruction: &Instruction,
    symbol_table: &HashMap<String, u16>,
) -> Option<u16> {
    match instruction {
        Instruction::A(reference) => Some(emit_a_instruction(reference, symbol_table)),
        Instruction::C(dest, expr, jump) => Some(emit_c_instruction(dest, expr, jump)),
        Instruction::L(_) => None,
    }
}

fn emit_a_instruction(reference: &Reference, symbol_table: &HashMap<String, u16>) -> u16 {
    match reference {
        Reference::Symbol(symbol) => *symbol_table.get(symbol).unwrap(),
        Reference::Address(address) => *address,
    }
}

fn emit_c_instruction(dest: &HashSet<Register>, expr: &Expr, jump: &Jump) -> u16 {
    (0b111 << 13) | (emit_expr(expr) << 6) | (emit_dest(dest) << 3) | emit_jump(jump)
}

fn emit_dest(dest: &HashSet<Register>) -> u16 {
    let mut result: u16 = 0;
    for register in dest {
        result |= match register {
            Register::A => 0b100,
            Register::D => 0b010,
            Register::M => 0b001,
        }
    }
    result
}

/*
notes on translation

!n = -(1+n) = -n-1
!0 = -1
-1&x = x&-1 = x

x = D
y = A (if 'a' flag is cleared)
    M (if 'a' flag is set)

a flag  -> set on any instruction involving M
        -> cleared on any instruction involving A
c flags -> zx nx zy ny f no
           zx = zero x
           nx = not (invert) x after zeroing
           zy = zero y
           ny = not (invert) y after zeroing
           f  -> 1 add
                 0 and
           no = not (invert) the output


Expr    zx nx zy ny f  no transliteration
0       1  0  1  0  1  0  0+0
1       1  1  1  1  1  1  !(-1+-1) = 1+1-1
-1      1  1  1  0  1  0  -1+0
D       0  0  1  1  0  0  D&-1
A   M   1  1  0  0  0  0  -1&A
!D      0  0  1  1  0  1  !(D&-1)
!A  !M  1  1  0  0  0  1  !(-1&A)
-D      0  0  1  1  1  1  !(D+-1) = -D+1-1
-A  -M  1  1  0  0  1  1  !(-1+A) = -A+1-a
D+1     0  1  1  1  1  1  !(!D+-1) = !(-D-1-1) = D+1+1-1
A+1 M+1 1  1  0  1  1  1  !(-1+!A) = !(-A-1-1) = A+1+1-1
D-1     0  0  1  1  1  0  D+-1
A-1 M-1 1  1  0  0  1  0  -1+A
D+A D+M 0  0  0  0  1  0  D+A
D-A D-M 0  1  0  0  1  1  !(!D+A) = !(A-D-1) = D-A+1-1
A-D M-D 0  0  0  1  1  1  !(D+!A) = !(D-A-1) = A-D+1-1
D&A D&M 0  0  0  0  0  0  D&A
D|A D|M 0  1  0  1  0  1  !(!D&!A)

*/

fn emit_expr(expr: &Expr) -> u16 {
    use Expr::*;
    match expr {
        Zero => 0b0101010,
        One => 0b0111111,
        NegOne => 0b0111010,
        D => 0b0001100,
        A => 0b0110000,
        M => 0b1110000,
        NotD => 0b0001101,
        NotA => 0b0110001,
        NotM => 0b1110001,
        NegD => 0b0001111,
        NegA => 0b0110011,
        NegM => 0b1110011,
        DAddOne => 0b0011111,
        AAddOne => 0b0110111,
        MAddOne => 0b1110111,
        DSubOne => 0b0001110,
        ASubOne => 0b0110010,
        MSubOne => 0b1110010,
        DAddA => 0b0000010,
        DAddM => 0b1000010,
        DSubA => 0b0010011,
        DSubM => 0b1010011,
        ASubD => 0b0000111,
        MSubD => 0b1000111,
        DAndA => 0b0000000,
        DAndM => 0b1000000,
        DOrA => 0b0010101,
        DOrM => 0b1010101,
    }
}

fn emit_jump(jump: &Jump) -> u16 {
    match jump {
        Jump::Null => 0b000,
        Jump::Jgt => 0b001,
        Jump::Jeq => 0b010,
        Jump::Jge => 0b011,
        Jump::Jlt => 0b100,
        Jump::Jne => 0b101,
        Jump::Jle => 0b110,
        Jump::Jmp => 0b111,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_emit_a_instruction() {
        let symbol_table = HashMap::from([("FOO".to_string(), 456), ("BAR".to_string(), 2)]);
        assert_eq!(
            emit_a_instruction(&Reference::Address(123), &symbol_table),
            123
        );
        assert_eq!(
            emit_a_instruction(&Reference::Symbol("FOO".to_string()), &symbol_table),
            456
        );
    }

    #[test]
    fn test_emit_jump() {
        assert_eq!(emit_jump(&Jump::Null), 0b000);
        assert_eq!(emit_jump(&Jump::Jgt), 0b001);
        assert_eq!(emit_jump(&Jump::Jeq), 0b010);
        assert_eq!(emit_jump(&Jump::Jge), 0b011);
        assert_eq!(emit_jump(&Jump::Jlt), 0b100);
        assert_eq!(emit_jump(&Jump::Jne), 0b101);
        assert_eq!(emit_jump(&Jump::Jle), 0b110);
        assert_eq!(emit_jump(&Jump::Jmp), 0b111);
    }

    #[test]
    fn test_emit_register() {
        assert_eq!(emit_dest(&HashSet::new()), 0b000);
        assert_eq!(emit_dest(&HashSet::from([Register::M])), 0b001);
        assert_eq!(emit_dest(&HashSet::from([Register::D])), 0b010);
        assert_eq!(emit_dest(&HashSet::from([Register::D, Register::M])), 0b011);
        assert_eq!(emit_dest(&HashSet::from([Register::A])), 0b100);
        assert_eq!(emit_dest(&HashSet::from([Register::A, Register::M])), 0b101);
        assert_eq!(emit_dest(&HashSet::from([Register::A, Register::D])), 0b110);
        assert_eq!(
            emit_dest(&HashSet::from([Register::A, Register::D, Register::M])),
            0b111
        );
    }

    #[test]
    fn test_emit_expr() {
        use Expr::*;
        assert_eq!(emit_expr(&Zero), 0b0101010);
        assert_eq!(emit_expr(&One), 0b0111111);
        assert_eq!(emit_expr(&NegOne), 0b0111010);
        assert_eq!(emit_expr(&D), 0b0001100);
        assert_eq!(emit_expr(&A), 0b0110000);
        assert_eq!(emit_expr(&M), 0b1110000);
        assert_eq!(emit_expr(&NotD), 0b0001101);
        assert_eq!(emit_expr(&NotA), 0b0110001);
        assert_eq!(emit_expr(&NotM), 0b1110001);
        assert_eq!(emit_expr(&NegD), 0b0001111);
        assert_eq!(emit_expr(&NegA), 0b0110011);
        assert_eq!(emit_expr(&NegM), 0b1110011);
        assert_eq!(emit_expr(&DAddOne), 0b0011111);
        assert_eq!(emit_expr(&AAddOne), 0b0110111);
        assert_eq!(emit_expr(&MAddOne), 0b1110111);
        assert_eq!(emit_expr(&DSubOne), 0b0001110);
        assert_eq!(emit_expr(&ASubOne), 0b0110010);
        assert_eq!(emit_expr(&MSubOne), 0b1110010);
        assert_eq!(emit_expr(&DAddA), 0b0000010);
        assert_eq!(emit_expr(&DAddM), 0b1000010);
        assert_eq!(emit_expr(&DSubA), 0b0010011);
        assert_eq!(emit_expr(&DSubM), 0b1010011);
        assert_eq!(emit_expr(&ASubD), 0b0000111);
        assert_eq!(emit_expr(&MSubD), 0b1000111);
        assert_eq!(emit_expr(&DAndA), 0b0000000);
        assert_eq!(emit_expr(&DAndM), 0b1000000);
        assert_eq!(emit_expr(&DOrA), 0b0010101);
        assert_eq!(emit_expr(&DOrM), 0b1010101);
    }

    #[test]
    fn test_emit_c_instruction() {
        assert_eq!(
            emit_c_instruction(
                &HashSet::from([Register::A, Register::D]),
                &Expr::DSubA,
                &Jump::Jne
            ),
            0b1110010011110101
        );
    }

    #[test]
    fn test_emit_instruction() {
        let symbol_table = HashMap::from([("FOO".to_string(), 456), ("BAR".to_string(), 2)]);
        assert_eq!(
            emit_instruction(&Instruction::L("BLAH".to_string()), &symbol_table),
            None
        );
        assert_eq!(
            emit_instruction(
                &Instruction::A(Reference::Symbol("FOO".to_string())),
                &symbol_table
            ),
            Some(456)
        );

        assert_eq!(
            emit_instruction(
                &Instruction::C(
                    HashSet::from([Register::A, Register::D]),
                    Expr::DSubA,
                    Jump::Jne,
                ),
                &symbol_table
            ),
            Some(0b1110010011110101)
        );
    }
}
