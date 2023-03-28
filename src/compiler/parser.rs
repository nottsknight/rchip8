use std::collections::VecDeque;

use regex::Regex;

use crate::machine::insts::Chip8Inst;

pub struct Parser<'a> {
    input: VecDeque<&'a str>,
    output: Vec<Chip8Inst>,
    gen_register_pattern: Regex,
    hex_value_pattern: Regex,
}

impl<'a> Parser<'a> {
    pub fn new(input_str: &'a str) -> Parser {
        Parser {
            input: tokenize(input_str),
            output: Vec::new(),
            gen_register_pattern: Regex::new(r"V[0-9a-fA-F]").unwrap(),
            hex_value_pattern: Regex::new(r"[0-9a-fA-F]+").unwrap(),
        }
    }

    pub fn run_parse(&mut self) -> Result<(), String> {
        loop {
            let inst = self.parse_inst()?;
            self.output.push(inst);
            if self.input.is_empty() {
                break;
            }
        }
        Ok(())
    }

    fn parse_inst(&mut self) -> Result<Chip8Inst, String> {
        if let Some(tok) = self.input.pop_front() {
            match tok {
                "mc" => {
                    let nnn = self.parse_lit_hex(3)?;
                    Ok(Chip8Inst::MachineInst(nnn as usize))
                }
                "clr" => Ok(Chip8Inst::ClearScreen),
                "retn" => Ok(Chip8Inst::SubReturn),
                "jmp" => {
                    let nnn = self.parse_lit_hex(3)?;
                    Ok(Chip8Inst::Jump(nnn as usize))
                }
                "call" => {
                    let nnn = self.parse_lit_hex(3)?;
                    Ok(Chip8Inst::SubCall(nnn as usize))
                }
                _ => Err(String::from("Unspecified parse error")),
            }
        } else {
            Err(String::from("Empty stack"))
        }
    }

    fn parse_lit_hex(&mut self, n: usize) -> Result<u16, String> {
        if self.input[0].len() != n {
            return Err(String::from("Read a number of incorrect length"));
        }

        let tok = self.input.pop_front().unwrap();
        u16::from_str_radix(&tok, 16).map_err(|e| e.to_string())
    }

    fn parse_comma(&mut self) -> Result<(), String> {
        let tok = self.input.pop_front().unwrap();
        return if tok == "," {
            Ok(())
        } else {
            Err(String::from("Expected comma"))
        };
    }

    fn parse_gen_reg(&mut self) -> Result<u8, String> {
        let tok = self.input.pop_front().unwrap();
        if self.gen_register_pattern.is_match(tok) {
            u8::from_str_radix(&tok[1..2], 16).map_err(|e| e.to_string())
        } else {
            Err(String::from("Expected general register"))
        }
    }

    fn parse_reg_reg_pair(&mut self) -> Result<(u8, u8), String> {
        let x = self.parse_gen_reg()?;
        self.parse_comma()?;
        let y = self.parse_gen_reg()?;
        Ok((x, y))
    }

    fn parse_reg_lit_pair(&mut self) -> Result<(u8, u8), String> {
        let x = self.parse_gen_reg()?;
        self.parse_comma()?;
        let nn = self.parse_lit_hex(2)?;
        Ok((x, nn as u8))
    }
}

fn tokenize(input: &str) -> VecDeque<&str> {
    let ws = Regex::new(r"\s+").unwrap();
    let mut toks = VecDeque::new();
    for tok in ws.split(input) {
        if tok.ends_with(',') {
            toks.push_back(&tok[..tok.len() - 1]);
            toks.push_back(",");
        } else {
            toks.push_back(tok);
        }
    }

    toks
}
