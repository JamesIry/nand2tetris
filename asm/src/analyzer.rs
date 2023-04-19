use std::collections::HashMap;

use crate::ast::{Instruction, Reference};

pub fn analyze<'a, T>(instructions: T) -> HashMap<String, u16>
where
    T: IntoIterator<Item = &'a Instruction>,
{
    let mut symbol_table = HashMap::new();
    let mut references = Vec::new();
    let mut addr = 0;
    for instruction in instructions.into_iter() {
        match instruction {
            Instruction::A(reference) => {
                match reference {
                    Reference::Address(_) => (),
                    Reference::Symbol(symbol) => {
                        references.push(symbol.clone());
                    }
                }

                addr += 1;
            }
            Instruction::C(_, _, _) => {
                addr += 1;
            }
            Instruction::L(symbol) => {
                // L instructions are virutal, and won't
                // be emmitted so don't need to increment the
                // addr
                symbol_table.insert(symbol.clone(), addr as u16);
            }
        }
    }

    let mut variable_addr = 16;
    for reference in references {
        #[allow(clippy::map_entry)]
        if !(symbol_table.contains_key(&reference)) {
            let (found_addr, new_variable_addr) = find_addr(&reference, variable_addr);
            symbol_table.insert(reference, found_addr);
            variable_addr = new_variable_addr;
        }
    }

    symbol_table
}

fn find_addr(reference: &str, variable_addr: u16) -> (u16, u16) {
    match reference {
        "R0" => (0, variable_addr),
        "R1" => (1, variable_addr),
        "R2" => (2, variable_addr),
        "R3" => (3, variable_addr),
        "R4" => (4, variable_addr),
        "R5" => (5, variable_addr),
        "R6" => (6, variable_addr),
        "R7" => (7, variable_addr),
        "R8" => (8, variable_addr),
        "R9" => (9, variable_addr),
        "R10" => (10, variable_addr),
        "R11" => (11, variable_addr),
        "R12" => (12, variable_addr),
        "R13" => (13, variable_addr),
        "R14" => (14, variable_addr),
        "R15" => (15, variable_addr),
        "SP" => (0, variable_addr),
        "LCL" => (1, variable_addr),
        "ARG" => (2, variable_addr),
        "THIS" => (3, variable_addr),
        "THAT" => (4, variable_addr),
        "SCREEN" => (16384, variable_addr),
        "KBD" => (24576, variable_addr),
        _ => (variable_addr, variable_addr + 1),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_addr() {
        assert_eq!(find_addr("R0", 16), (0, 16));
        assert_eq!(find_addr("R1", 16), (1, 16));
        assert_eq!(find_addr("R2", 16), (2, 16));
        assert_eq!(find_addr("R3", 16), (3, 16));
        assert_eq!(find_addr("R4", 16), (4, 16));
        assert_eq!(find_addr("R5", 16), (5, 16));
        assert_eq!(find_addr("R6", 16), (6, 16));
        assert_eq!(find_addr("R7", 16), (7, 16));
        assert_eq!(find_addr("R8", 16), (8, 16));
        assert_eq!(find_addr("R9", 16), (9, 16));
        assert_eq!(find_addr("R10", 16), (10, 16));
        assert_eq!(find_addr("R11", 16), (11, 16));
        assert_eq!(find_addr("R12", 16), (12, 16));
        assert_eq!(find_addr("R13", 16), (13, 16));
        assert_eq!(find_addr("R14", 16), (14, 16));
        assert_eq!(find_addr("R15", 16), (15, 16));
        assert_eq!(find_addr("SP", 16), (0, 16));
        assert_eq!(find_addr("LCL", 16), (1, 16));
        assert_eq!(find_addr("ARG", 16), (2, 16));
        assert_eq!(find_addr("THIS", 16), (3, 16));
        assert_eq!(find_addr("THAT", 16), (4, 16));
        assert_eq!(find_addr("SCREEN", 16), (16384, 16));
        assert_eq!(find_addr("KBD", 16), (24576, 16));
        assert_eq!(find_addr("FOO", 16), (16, 17));
        assert_eq!(find_addr("BAR", 200), (200, 201));
    }
}
