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
    let f = OpenOptions::new().create_new(true).open(filename)?;
    let mut w = BufWriter::new(f);
    for c in code {
        let c1 = c.to_be_bytes();
        w.write_all(&c1)?;
    }
    w.flush()
}

fn compile(infile: &str, outfile: &str) -> std::io::Result<()> {
    let text = load_file(infile)?;
    match parse(&text) {
        Ok(code) => emit_code(outfile, code),
        Err(e) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            e.to_string(),
        )),
    }
}

#[derive(Parser, Debug)]
struct CompileArgs {
    file: String,
    #[arg(short, default_value = "a.out")]
    outfile: String,
}

fn main() {
    let args = CompileArgs::parse();
    compile(&args.file, &args.outfile).unwrap();
}
