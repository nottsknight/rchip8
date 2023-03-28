use crate::machine::insts::Chip8Inst;
use std::fs::File;
use std::io::{BufWriter, Result, Write};

mod parser;
mod inst_into_u16;


pub fn emit_code(filename: &str, insts: Vec<Chip8Inst>) -> Result<()> {
    let f = File::open(filename)?;
    let mut w = BufWriter::new(f);
    for i in insts {
        let op = u16::from(i);
        w.write(&op.to_be_bytes()[..])?;
    }
    w.flush()?;
    Ok(())
}
