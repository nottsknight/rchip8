use super::ast::{Expression, Statement};
use crate::machine::insts::Chip8Inst;
use std::collections::HashMap;

trait IntoCode {
    fn into_code(self: &Self, gen: &PseudoRegGenerator) -> Vec<Chip8Inst>;
}

struct PseudoRegGenerator {
    next_reg: usize,
    assigned_regs: HashMap<String, usize>,
    next_label: usize,
}

impl PseudoRegGenerator {
    fn new() -> PseudoRegGenerator {
        PseudoRegGenerator {
            next_reg: 0,
            assigned_regs: HashMap::new(),
            next_label: 0,
        }
    }

    fn next_reg(&mut self) -> usize {
        let r = self.next_reg;
        self.next_reg += 1;
        r
    }

    fn next_var_reg(&mut self, var_name: &str) -> usize {
        if let Some(r) = self.assigned_regs.get(var_name) {
            return *r;
        }

        let r = self.next_reg();
        self.assigned_regs.insert(String::from(var_name), r);
        r
    }
    
    fn next_label(&mut self) -> usize {
        let l = self.next_label;
        self.next_label += 1;
        l
    }
}