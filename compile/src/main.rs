use anyhow::Result;
use std::{
    env::args,
    fs::File,
    io::{prelude::*, BufReader, BufWriter},
    path::{Path, PathBuf},
};
use utf8_chars::BufReadCharsExt;

use crate::tokenizer::Tokenizer;

mod tokenizer;

fn main() -> Result<()> {
    if args().len() < 2 {
        println!("missing file name")
    } else {
        let args = args().collect::<Vec<_>>();
        let output_tokens = args.len() >= 3 && args[2] == "-output-tokens";
        let input_name = args[1].as_str();
        let input_path = Path::new(input_name);
        let (input_files, output_path) = create_output_path(input_path);
        println!("Creating {}", output_path.to_string_lossy());
        let output_file = File::create(&output_path)?;

        for input_file in input_files {
            compile_file(input_file.as_path(), &output_file, output_tokens)?;
        }
    }

    Ok(())
}

fn compile_file(input_path: &Path, _output_file: &File, output_tokens: bool) -> Result<()> {
    println!("Compiling {}", input_path.to_string_lossy());

    let input_file = File::open(input_path)?;
    let mut reader = BufReader::new(input_file);

    let tokens = Tokenizer::new(reader.chars());
    if output_tokens {
        let mut path = input_path.to_path_buf();
        path.set_extension("");
        let mut name = path.file_name().unwrap().to_string_lossy().to_string();
        name.push('T');
        path.set_file_name(name);
        path.set_extension("gen.xml");

        println!("Creating {}", path.to_string_lossy());
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        writeln!(writer, "<tokens>")?;
        for token in tokens {
            writeln!(
                writer,
                "{}",
                match token? {
                    tokenizer::Token::IntegerLiteral(integer) => {
                        xml("integerConstant", &integer.to_string())
                    }
                    tokenizer::Token::StringLiteral(string) => xml("stringConstant", &string),
                    tokenizer::Token::Identifier(string) => xml("identifier", &string),
                    tokenizer::Token::Symbol(c) => xml("symbol", &c.to_string()),
                    tokenizer::Token::Keyword(string) => xml("keyword", string),
                }
            )?;
        }
        writeln!(writer, "</tokens>")?;
    }

    Ok(())
}

fn xml(element: &str, string: &str) -> String {
    let string = string.replace('&', "&amp;");
    let string = string.replace('\'', "&quot;");
    let string = string.replace('<', "&lt;");
    let string = string.replace('>', "&gt;");

    let mut result = "<".to_string();
    result.push_str(element);
    result.push_str("> ");

    result.push_str(&string);

    result.push_str(" </");
    result.push_str(element);
    result.push('>');

    result
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
                    if extn.to_string_lossy().as_ref() == "jack" {
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
    output_path.set_extension("vm");
    (files, output_path)
}
