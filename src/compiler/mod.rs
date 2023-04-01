use std::collections::HashMap;

pub enum ProgElement {
    Word(u8),
    DWord(u16),
    Instr(u16),
    Jump(String),
    Call(String),
    JumpV(String),
    LabelInstr(String, Box<ProgElement>),
}

fn bytes(n: u16) -> (u8, Option<u8>) {
    (((n & 0xff00) >> 8) as u8, Some((n & 0xff) as u8))
}

impl ProgElement {
    fn into_bytes(&self, locs: &HashMap<&String, u16>) -> (u8, Option<u8>) {
        match self {
            ProgElement::LabelInstr(_, elem) => elem.into_bytes(locs),
            ProgElement::Instr(op) => bytes(*op),
            ProgElement::Word(data) => (*data, None),
            ProgElement::DWord(data) => bytes(*data),
            ProgElement::Jump(loc) => {
                if let Some(addr) = locs.get(loc) {
                    bytes(0x1000 | addr)
                } else {
                    panic!("Jump to undefined location: {}", loc);
                }
            }
            ProgElement::Call(loc) => {
                if let Some(addr) = locs.get(loc) {
                    bytes(0x2000 | addr)
                } else {
                    panic!("Call to undefined location: {}", loc);
                }
            }
            ProgElement::JumpV(loc) => {
                if let Some(addr) = locs.get(loc) {
                    bytes(0xb000 | addr)
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
        match elem {
            ProgElement::Word(_) => pc += 1,
            _ => pc += 2,
        }
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
            ProgElement::Word(2),
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

pub fn process_prog(prog: Vec<ProgElement>) -> Vec<u8> {
    let lbls = label_addresses(&prog);
    let mut bytes = Vec::new();
    for elem in &prog {
        let (b1, b2opt) = elem.into_bytes(&lbls);
        bytes.push(b1);
        if let Some(b2) = b2opt {
            bytes.push(b2);
        }
    }
    bytes
}
