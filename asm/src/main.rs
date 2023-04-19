use std::{
    env::args,
    fs::File,
    io::{prelude::*, BufRead, BufReader, BufWriter},
};

mod analyzer;
mod ast;
mod emitter;
mod parser;

fn main() -> std::io::Result<()> {
    if args().len() < 2 {
        println!("missing file name")
    } else {
        let input_filename = args().nth(1).unwrap();
        println!("Assembling {input_filename}");
        let input_file = File::open(&input_filename)?;
        let reader = BufReader::new(input_file);
        let lines = reader.lines().map(|line| line.unwrap());
        let results = parser::parse_lines(lines);
        match results {
            Err(errors) => {
                errors.iter().for_each(|error| println!("{:?}", error));
            }
            Ok(instructions) => {
                let symbol_table = analyzer::analyze(instructions.iter());
                let codes = emitter::emit_instructions(instructions.iter(), &symbol_table);

                let output_filename = create_output_filename(&input_filename);
                println!("Creating {output_filename}");
                let output_file = File::create(output_filename)?;
                let mut writer = BufWriter::new(output_file);
                for code in codes {
                    writeln!(writer, "{}", bits(code))?;
                }
            }
        };
    }
    Ok(())
}

fn create_output_filename(input_filename: &str) -> String {
    let dot_index = input_filename.rfind('.');
    let base_output_filename = dot_index
        .map(|idx| &input_filename[..idx])
        .unwrap_or(input_filename);

    let mut output_filename = base_output_filename.to_string();
    output_filename.push_str(".hack");
    output_filename
}

fn bits(input: u16) -> String {
    let mut output = String::with_capacity(16);
    for i in (0..=15u16).rev() {
        let bit = 1u16 << i;
        if (input & bit) != 0 {
            output.push('1');
        } else {
            output.push('0');
        }
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_output_filename() {
        assert_eq!(
            create_output_filename("../whatever/foo.asm"),
            "../whatever/foo.hack".to_string()
        );
    }

    #[test]
    fn test_bits() {
        assert_eq!(bits(0b1001011101011001), "1001011101011001");
    }
}
