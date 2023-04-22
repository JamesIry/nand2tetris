mod ast;
mod emitter;
mod parser;
mod printer;

use std::{
    env::args,
    fs::File,
    io::{prelude::*, BufRead, BufReader, BufWriter},
    path::{Path, PathBuf},
};

fn main() -> std::io::Result<()> {
    if args().len() < 2 {
        println!("missing file name")
    } else {
        let input_name = args().nth(1).unwrap();
        let input_path = Path::new(&input_name);
        let (input_files, output_path) = create_output_path(input_path);
        println!("Creating {}", output_path.to_string_lossy());
        let output_file = File::create(&output_path)?;

        let mut boostrap = input_files.len() > 1;

        for input_file in input_files {
            translate_file(input_file.as_path(), &output_file, boostrap)?;
            boostrap = false;
        }
    }

    Ok(())
}

fn translate_file(
    input_path: &Path,
    output_file: &File,
    bootstrap: bool,
) -> Result<(), std::io::Error> {
    println!("Translating {}", input_path.to_string_lossy());
    let statics_base = input_path.file_stem().unwrap().to_string_lossy();

    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);
    let lines = reader.lines().map(|line| line.unwrap());
    let results = parser::parse_lines(lines);
    match results {
        Err(errors) => {
            errors.iter().for_each(|error| println!("{:?}", error));
        }
        Ok(commands) => {
            let asm = emitter::emit_commands(commands, &statics_base, bootstrap);

            let mut writer = BufWriter::new(output_file);
            for s in asm {
                writeln!(writer, "{}", s)?;
            }
        }
    };
    Ok(())
}

fn create_output_path(input_path: &Path) -> (Vec<PathBuf>, PathBuf) {
    let base_name = input_path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    let (files, mut output_path) = if input_path.is_file() {
        (vec![PathBuf::from(input_path)], PathBuf::from(input_path))
    } else if input_path.is_dir() {
        let files = input_path
            .read_dir()
            .unwrap()
            .filter_map(|p| {
                let file = p.unwrap().path();
                file.extension().and_then(|extn| {
                    if extn.to_string_lossy().as_ref() == "vm" {
                        Some(file.clone())
                    } else {
                        None
                    }
                })
            })
            .collect();

        let output_path = PathBuf::from(input_path).join(base_name);
        (files, output_path)
    } else {
        panic!("Unable to find {}.", input_path.to_string_lossy());
    };
    output_path.set_extension("asm");
    (files, output_path)
}
