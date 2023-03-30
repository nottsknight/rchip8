use std::collections::HashMap;

pub enum ProgElement {
    Instr(u16),
    LabelInstr(String, u16),
    Jump(String),
    Call(String),
    JumpV(String),
    Data(u16),
}

fn label_addresses(elems: &Vec<ProgElement>) -> HashMap<&String, u16> {
    let mut addrs = HashMap::new();
    let mut pc = 0x200;
    for elem in elems {
        if let ProgElement::LabelInstr(lbl, _) = elem {
            addrs.insert(lbl, pc);
        }
        pc += 2;
    }
    addrs
}

pub fn process_prog(prog: Vec<ProgElement>) -> Vec<u16> {
    let lbls = label_addresses(&prog);
    prog.iter()
        .map(|elem| match elem {
            ProgElement::Instr(x) => ProgElement::Instr(x.clone()),
            ProgElement::LabelInstr(_, x) => ProgElement::Instr(x.clone()),
            ProgElement::Data(x) => ProgElement::Data(x.clone()),
            ProgElement::Jump(lbl) => {
                if let Some(addr) = lbls.get(&lbl) {
                    ProgElement::Instr(0x1000 | addr)
                } else {
                    panic!("Jump to undefined label: {}", lbl);
                }
            }
            ProgElement::Call(lbl) => {
                if let Some(addr) = lbls.get(&lbl) {
                    ProgElement::Instr(0x2000 | addr)
                } else {
                    panic!("Call invalid label: {}", lbl);
                }
            }
            ProgElement::JumpV(lbl) => {
                if let Some(addr) = lbls.get(&lbl) {
                    ProgElement::Instr(0xb000 | addr)
                } else {
                    panic!("Jump to invalid label: {}", lbl);
                }
            }
        })
        .map(|elem| match elem {
            ProgElement::Instr(x) => x,
            ProgElement::Data(x) => x,
            _ => 0,
        })
        .collect()
}
