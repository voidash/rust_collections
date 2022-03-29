use crate::ast::{Opcode, Op};


pub fn assemble(parsed_line: Opcode) {
    let mut assembled_vec:Vec<u8> = Vec::new(); 

    match(parsed_line.Op) {
        Op::OpR(instruction, register) => {
            let ins = &instruction[..];
            let reg = &register[..];
            match(ins, reg) {
                ("inr", a) => {
                    match a {
                        "b" => { assembled_vec.push(0x04); },
                        "c" => { assembled_vec.push(0x0c); },
                        "d" => { assembled_vec.push(0x14); },
                        "e" => { assembled_vec.push(0x1c); },
                        "h" => { assembled_vec.push(0x24); },
                        "l" => { assembled_vec.push(0x2c); },
                        "m" => { assembled_vec.push(0x34); },
                        _ => { }
                    }
                },
                ("dcr", a) => {
                    match a {
                        "b" => { assembled_vec.push(0x05); },
                        "c" => { assembled_vec.push(0x0d); },
                        "d" => { assembled_vec.push(0x15); },
                        "e" => { assembled_vec.push(0x1d); },
                        "h" => { assembled_vec.push(0x24); },
                        "l" => { assembled_vec.push(0x2d); },
                        "m" => { assembled_vec.push(0x35); },
                        _ => { }
                    }
                },
                ("inx", a) => {
                    match a {
                        "b" => { assembled_vec.push(0x03); },
                        "d" => { assembled_vec.push(0x13); },
                        "h" => { assembled_vec.push(0x23); },
                        "sp" => { assembled_vec.push(0x33); },
                        _ => { }
                    }
                },
                ("dcx", a) => {
                    match a {
                        "b" => { assembled_vec.push(0x0b); },
                        "d" => { assembled_vec.push(0x1b); },
                        "h" => { assembled_vec.push(0x2b); },
                        "sp" => { assembled_vec.push(0x3b); },
                        _ => { }
                    }
                },
                ("add", a) => {
                    match a {
                        "b" => { assembled_vec.push(0x80); },
                        "c" => { assembled_vec.push(0x81); },
                        "d" => { assembled_vec.push(0x82); },
                        "e" => { assembled_vec.push(0x83); },
                        "h" => { assembled_vec.push(0x84); },
                        "l" => { assembled_vec.push(0x85); },
                        "m" => { assembled_vec.push(0x86); },
                        "a" => { assembled_vec.push(0x87); },
                        _ => { }
                    }
                },
                ("sub", a) => {
                    match a {
                        "b" => { assembled_vec.push(0x90); },
                        "c" => { assembled_vec.push(0x91); },
                        "d" => { assembled_vec.push(0x92); },
                        "e" => { assembled_vec.push(0x93); },
                        "h" => { assembled_vec.push(0x94); },
                        "l" => { assembled_vec.push(0x95); },
                        "m" => { assembled_vec.push(0x96); },
                        "a" => { assembled_vec.push(0x97); },
                        _ => { }
                    }
                },
                ("adc", a) => {
                    match a {
                        "b" => { assembled_vec.push(0x88); },
                        "c" => { assembled_vec.push(0x89); },
                        "d" => { assembled_vec.push(0x8a); },
                        "e" => { assembled_vec.push(0x8b); },
                        "h" => { assembled_vec.push(0x8c); },
                        "l" => { assembled_vec.push(0x8d); },
                        "m" => { assembled_vec.push(0x8e); },
                        "a" => { assembled_vec.push(0x8f); },
                        _ => { }
                    }
                },
                ("sbb", a) => {
                    match a {
                        "b" => { assembled_vec.push(0x98); },
                        "c" => { assembled_vec.push(0x99); },
                        "d" => { assembled_vec.push(0x9a); },
                        "e" => { assembled_vec.push(0x9b); },
                        "h" => { assembled_vec.push(0x9c); },
                        "l" => { assembled_vec.push(0x9d); },
                        "m" => { assembled_vec.push(0x9e); },
                        "a" => { assembled_vec.push(0x9f); },
                        _ => { }
                    }
                },
                ("ana", a) => {
                    match a {
                        "b" => { assembled_vec.push(0xa0); },
                        "c" => { assembled_vec.push(0xa1); },
                        "d" => { assembled_vec.push(0xa2); },
                        "e" => { assembled_vec.push(0xa3); },
                        "h" => { assembled_vec.push(0xa4); },
                        "l" => { assembled_vec.push(0xa5); },
                        "m" => { assembled_vec.push(0xa6); },
                        "a" => { assembled_vec.push(0xa7); },
                        _ => { }
                    }
                },
                ("xra", a) => {
                    match a {
                        "b" => { assembled_vec.push(0xa8); },
                        "c" => { assembled_vec.push(0xa9); },
                        "d" => { assembled_vec.push(0xaa); },
                        "e" => { assembled_vec.push(0xab); },
                        "h" => { assembled_vec.push(0xac); },
                        "l" => { assembled_vec.push(0xad); },
                        "m" => { assembled_vec.push(0xae); },
                        "a" => { assembled_vec.push(0xaf); },
                        _ => { }
                    }
                },
                ("ora", a) => {
                    match a {
                        "b" => { assembled_vec.push(0xb0); },
                        "c" => { assembled_vec.push(0xb1); },
                        "d" => { assembled_vec.push(0xb2); },
                        "e" => { assembled_vec.push(0xb3); },
                        "h" => { assembled_vec.push(0xb4); },
                        "l" => { assembled_vec.push(0xb5); },
                        "m" => { assembled_vec.push(0xb6); },
                        "a" => { assembled_vec.push(0xb7); },
                        _ => { }
                    }
                },
                ("cmp", a) => {
                    match a {
                        "b" => { assembled_vec.push(0xb8); },
                        "c" => { assembled_vec.push(0xb9); },
                        "d" => { assembled_vec.push(0xba); },
                        "e" => { assembled_vec.push(0xbb); },
                        "h" => { assembled_vec.push(0xbc); },
                        "l" => { assembled_vec.push(0xbd); },
                        "m" => { assembled_vec.push(0xbe); },
                        "a" => { assembled_vec.push(0xbf); },
                        _ => { }
                    }
                },
                ("stax", a) => {
                    match a {
                        "b" => { assembled_vec.push(0x02); },
                        "d" => { assembled_vec.push(0x12); },
                        _ => { }
                    }
                },
                ("ldax", a) => {
                    match a {
                        "b" => { assembled_vec.push(0x0a); },
                        "d" => { assembled_vec.push(0x1a); },
                        _ => { }
                    }
                },
                (_,_) => {}
            }
        }
        _ => {}
    }
}
