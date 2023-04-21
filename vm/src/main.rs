mod ast;
mod emitter;
mod parser;
mod printer;

use std::{
    env::args,
    fs::File,
    io::{prelude::*, BufRead, BufReader, BufWriter},
    path::Path,
};

fn main() -> std::io::Result<()> {
    if args().len() < 2 {
        println!("missing file name")
    } else {
        let input_filename = args().nth(1).unwrap();
        println!("Translating {input_filename}");
        let input_file = File::open(&input_filename)?;
        let reader = BufReader::new(input_file);
        let lines = reader.lines().map(|line| line.unwrap());
        let results = parser::parse_lines(lines);
        match results {
            Err(errors) => {
                errors.iter().for_each(|error| println!("{:?}", error));
            }
            Ok(commands) => {
                let base_name = create_base_name(&input_filename);
                let asm = emitter::emit_commands(commands, &base_name);
                let output_filename = create_output_filename(&input_filename);
                println!("Creating {output_filename}");
                let output_file = File::create(output_filename)?;
                let mut writer = BufWriter::new(output_file);
                for s in asm {
                    writeln!(writer, "{}", s)?;
                }
            }
        }
    }
    Ok(())
}

fn create_output_filename(input_filename: &str) -> String {
    let dot_index = input_filename.rfind('.');
    let base_output_filename = dot_index
        .map(|idx| &input_filename[..idx])
        .unwrap_or(input_filename);

    let mut output_filename = base_output_filename.to_string();
    output_filename.push_str(".asm");
    output_filename
}

fn create_base_name(input_filename: &str) -> String {
    let path = Path::new(input_filename);
    path.file_stem().unwrap().to_string_lossy().into_owned()
}
