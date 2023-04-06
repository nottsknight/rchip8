// This file is part of rchip8.
//
// rchip8 is free software: you can redistribute it and/or modify it under the terms of
// the GNU General Public License as published by the Free Software Foundation, either
// version 3 of the License, or (at your option) any later version.
//
// rchip8 is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with rchip8.
// If not, see <https://www.gnu.org/licenses/>.use clap::Parser;

use clap::Parser;
use lalrpop_util::{lalrpop_mod, lexer::Token, ParseError};
use rchip8::compiler::process_prog;
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

fn parse(input: &str) -> Result<Vec<u8>, ParseError<usize, Token, &str>> {
    let parser = ProgramParser::new();
    let code = parser.parse(input)?;
    Ok(process_prog(code))
}

fn emit_code(filename: &str, code: Vec<u8>) -> std::io::Result<()> {
    let f = OpenOptions::new().write(true).create(true).open(filename)?;
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
