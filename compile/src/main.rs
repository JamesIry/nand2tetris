use anyhow::Result;
use std::{
    env::args,
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};
use utf8_chars::BufReadCharsExt;

use crate::{parser::Parser, tokenizer::Tokenizer};

mod ast;
mod parser;
mod tokenizer;
mod xml;

use crate::xml::*;

#[derive(Clone, Copy)]
enum Output {
    Tokens,
    Ast,
    Vm,
}

fn main() -> Result<()> {
    if args().len() < 2 {
        println!("missing file name")
    } else {
        let args = args().collect::<Vec<_>>();
        let output = if args.len() >= 3 {
            match args[2].as_str() {
                "-output=tokens" => Output::Tokens,
                "-output=ast" => Output::Ast,
                "-output=vm" => Output::Vm,
                _ => panic!("Unrecognized option {0}", args[2]),
            }
        } else {
            Output::Vm
        };
        let input_name = args[1].as_str();
        let input_path = Path::new(input_name);
        let (input_files, output_path) = create_output_path(input_path);
        println!("Creating {}", output_path.to_string_lossy());
        let output_file = File::create(&output_path)?;

        for input_file in input_files {
            compile_file(input_file.as_path(), &output_file, output)?;
        }
    }

    Ok(())
}

fn compile_file(input_path: &Path, _output_file: &File, output: Output) -> Result<()> {
    println!("Compiling {}", input_path.to_string_lossy());

    let input_file = File::open(input_path)?;
    let mut reader = BufReader::new(input_file);

    let tokens = Tokenizer::new(reader.chars());
    match output {
        Output::Tokens => {
            let mut path = input_path.to_path_buf();
            path.set_extension("");
            let mut name = path.file_name().unwrap().to_string_lossy().to_string();
            name.push('T');
            path.set_file_name(name);
            path.set_extension("gen.xml");

            println!("Creating {}", path.to_string_lossy());
            let file = File::create(path)?;
            let writer = BufWriter::new(file);
            let mut xml = Xml::new(writer);
            xml.start(0, "tokens")?;
            for token in tokens {
                let (element, value) = match token? {
                    tokenizer::Token::IntegerLiteral(integer) => {
                        ("integerConstant", integer.to_string())
                    }
                    tokenizer::Token::StringLiteral(string) => ("stringConstant", string),
                    tokenizer::Token::Identifier(string) => ("identifier", string),
                    tokenizer::Token::Symbol(c) => ("symbol", c.to_string()),
                    tokenizer::Token::Keyword(string) => ("keyword", string.to_string()),
                };
                xml.leaf(0, element, &value)?;
            }
            xml.end(0, "tokens")?;
            Ok(())
        }
        Output::Ast => {
            let mut parser = Parser::new(tokens);
            let ast = parser.parse_class().unwrap();

            let mut path = input_path.to_path_buf();
            path.set_extension("gen.xml");

            println!("Creating {}", path.to_string_lossy());
            let file = File::create(path)?;
            let writer = BufWriter::new(file);
            let mut xml = Xml::new(writer);
            xml.write_ast(ast)?;
            Ok(())
        }
        Output::Vm => {
            todo!()
        }
    }
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
