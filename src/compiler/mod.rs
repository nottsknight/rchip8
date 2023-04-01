use std::collections::HashMap;

pub enum ProgElement {
    Data(u16),
    Instr(u16),
    Jump(String),
    Call(String),
    JumpV(String),
    LabelInstr(String, Box<ProgElement>),
}

impl ProgElement {
    fn into_u16(&self, locs: &HashMap<&String, u16>) -> u16 {
        match self {
            ProgElement::LabelInstr(_, elem) => elem.into_u16(locs),
            ProgElement::Instr(op) => *op,
            ProgElement::Data(data) => *data,
            ProgElement::Jump(loc) => {
                if let Some(addr) = locs.get(loc) {
                    0x1000 | addr
                } else {
                    panic!("Jump to undefined location: {}", loc);
                }
            }
            ProgElement::Call(loc) => {
                if let Some(addr) = locs.get(loc) {
                    0x2000 | addr
                } else {
                    panic!("Call to undefined location: {}", loc);
                }
            }
            ProgElement::JumpV(loc) => {
                if let Some(addr) = locs.get(loc) {
                    0xb000 | addr
                } else {
                    panic!("Jump to undefined location: {}", loc);
                }
            }
        }
    }
}

fn label_addresses(elems: &Vec<ProgElement>) -> HashMap<&String, u16> {
    let mut addrs = HashMap::new();
    let mut pc = 0x200;
    for elem in elems {
        if let ProgElement::LabelInstr(lbl, _) = elem {
            if addrs.contains_key(lbl) {
                panic!("Duplicate label: {}", lbl);
            }
            addrs.insert(lbl, pc);
        }
        pc += 2;
    }
    addrs
}

#[cfg(test)]
mod label_addresses_tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_no_duplicate_labels() {
        let elems = vec![
            ProgElement::Instr(0),
            ProgElement::LabelInstr(String::from("l1"), Box::new(ProgElement::Instr(0))),
            ProgElement::Data(2),
            ProgElement::LabelInstr(String::from("l2"), Box::new(ProgElement::Instr(0))),
            ProgElement::Jump(String::from("l3")),
        ];

        let lbls = label_addresses(&elems);
        assert_eq!(2, lbls.len());
        let k1 = String::from("l1");
        assert!(lbls.contains_key(&k1));
        let k2 = String::from("l2");
        assert!(lbls.contains_key(&k2));
    }

    #[rstest]
    #[should_panic]
    fn test_duplicate_labels() {
        let elems = vec![
            ProgElement::LabelInstr(String::from("l1"), Box::new(ProgElement::Instr(0))),
            ProgElement::LabelInstr(String::from("l1"), Box::new(ProgElement::Instr(0))),
        ];

        label_addresses(&elems);
    }
}

pub fn process_prog(prog: Vec<ProgElement>) -> Vec<u16> {
    let lbls = label_addresses(&prog);
    prog.iter().map(|elem| elem.into_u16(&lbls)).collect()
}
