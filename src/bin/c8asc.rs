use clap::Parser;
use lalrpop_util::lexer::Token;
use lalrpop_util::{lalrpop_mod, ParseError};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};

lalrpop_mod!(c8asm);
use c8asm::ProgramParser;

fn load_file(filename: &str) -> std::io::Result<String> {
    let f = File::open(filename)?;
    let mut r = BufReader::new(f);
    let mut buf = String::new();
    r.read_to_string(&mut buf)?;
    Ok(buf)
}

fn parse(input: &str) -> Result<Vec<u16>, ParseError<usize, Token, &str>> {
    let parser = ProgramParser::new();
    parser.parse(input)
}

fn emit_code(filename: &str, code: Vec<u16>) -> std::io::Result<()> {
    let f = OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename)?;
    let mut w = BufWriter::new(f);
    for c in code {
        let c1 = c.to_be_bytes();
        w.write_all(&c1)?;
    }
    w.flush()
}

#[derive(Parser)]
struct CompileArgs {
    /// Assembly file to compile
    file: String,
    /// Name of ROM file to generate
    #[arg(short, default_value = "a.out")]
    outfile: String,
}

fn main() {
    let args = CompileArgs::parse();
    match load_file(&args.file) {
        Err(e) => {
            println!("Couldn't load file: {:?}", e);
            std::process::exit(1);
        }
        Ok(text) => match parse(&text) {
            Err(e) => {
                println!("Failed to parse: {:?}", e);
                std::process::exit(2);
            }
            Ok(code) => match emit_code(&args.outfile, code) {
                Err(e) => {
                    println!("Couldn't emit code: {:?}", e);
                    std::process::exit(1);
                }
                Ok(_) => (),
            },
        },
    }
}
