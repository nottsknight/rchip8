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
// If not, see <https://www.gnu.org/licenses/>.

use std::collections::HashMap;

pub enum ProgElement {
    Data(Vec<u8>),
    Instr(u16),
    Jump(String),
    Call(String),
    JumpV(String),
    LabelInstr(String, Box<ProgElement>),
}

impl ProgElement {
    fn into_bytes(&self, locs: &HashMap<String, u16>) -> Vec<u8> {
        match self {
            ProgElement::LabelInstr(_, elem) => elem.into_bytes(locs),
            ProgElement::Instr(op) => Vec::from(op.to_be_bytes()),
            ProgElement::Data(data) => data.clone(),
            ProgElement::Jump(loc) => {
                if let Some(addr) = locs.get(loc) {
                    Vec::from((0x1000 | addr).to_be_bytes())
                } else {
                    panic!("Jump to undefined location: {}", loc);
                }
            }
            ProgElement::Call(loc) => {
                if let Some(addr) = locs.get(loc) {
                    Vec::from((0x2000 | addr).to_be_bytes())
                } else {
                    panic!("Call to undefined location: {}", loc);
                }
            }
            ProgElement::JumpV(loc) => {
                if let Some(addr) = locs.get(loc) {
                    Vec::from((0xb000 | addr).to_be_bytes())
                } else {
                    panic!("Jump to undefined location: {}", loc);
                }
            }
        }
    }
}

#[cfg(test)]
mod into_bytes_tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn empty_locs() -> HashMap<String, u16> {
        HashMap::new()
    }

    #[rstest]
    fn test_instr_into_bytes(empty_locs: HashMap<String, u16>) {
        let inst = ProgElement::Instr(0xa1b2);
        assert_eq!(vec![0xa1, 0xb2], inst.into_bytes(&empty_locs));
    }

    #[rstest]
    fn test_data_into_bytes(empty_locs: HashMap<String, u16>) {
        let inst = ProgElement::Data(vec![1, 2, 3, 4, 5]);
        assert_eq!(vec![1, 2, 3, 4, 5], inst.into_bytes(&empty_locs));
    }

    #[rstest]
    fn test_jump_into_bytes(mut empty_locs: HashMap<String, u16>) {
        empty_locs.insert(String::from("test"), 0x123);
        let inst = ProgElement::Jump(String::from("test"));
        assert_eq!(vec![0x11, 0x23], inst.into_bytes(&empty_locs));
    }

    #[rstest]
    fn test_call_into_bytes(mut empty_locs: HashMap<String, u16>) {
        empty_locs.insert(String::from("test"), 0xa14);
        let inst = ProgElement::Call(String::from("test"));
        assert_eq!(vec![0x2a, 0x14], inst.into_bytes(&empty_locs));
    }

    #[rstest]
    fn test_jumpv_into_bytes(mut empty_locs: HashMap<String, u16>) {
        empty_locs.insert(String::from("test"), 0x33e);
        let inst = ProgElement::JumpV(String::from("test"));
        assert_eq!(vec![0xb3, 0x3e], inst.into_bytes(&empty_locs));
    }
}

fn label_addresses(elems: &Vec<ProgElement>) -> HashMap<String, u16> {
    let mut addrs = HashMap::new();
    let mut pc = 0x200;
    for elem in elems {
        if let ProgElement::LabelInstr(lbl, _) = elem {
            if addrs.contains_key(lbl) {
                panic!("Duplicate label: {}", lbl);
            }
            addrs.insert(lbl.clone(), pc as u16);
        }
        match elem {
            ProgElement::Data(bytes) => pc += bytes.len(),
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
            ProgElement::Data(vec![2, 4]),
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
    for elem in prog {
        let mut elem_bytes = elem.into_bytes(&lbls);
        bytes.append(&mut elem_bytes);
    }
    bytes
}
