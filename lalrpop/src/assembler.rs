use crate::ast::{Opcode, Op};
use std::collections::HashMap;

pub fn assemble(parsed_line: Opcode, label_offset_map: HashMap<String,u32>) {
    let mut assembled_vec:Vec<u8> = Vec::new(); 

    match parsed_line.Op {
        Op::OpL(instruction,label) => {
            let ins = &instruction[..];
            if let Some(offset) = label_offset_map.get(&label) {
                match ins {
                    "jc" => {assembled_vec.push(0xda);},
                    "jnc" =>{assembled_vec.push(0xd2);},
                    "jz" => {assembled_vec.push(0xca);},
                    "jnz" => {assembled_vec.push(0xc2);},
                    "jp" => {assembled_vec.push(0xf2);},
                    "jm" => {assembled_vec.push(0xfa);},
                    "jpe" => {assembled_vec.push(0xea);},
                    "jpo" => {assembled_vec.push(0xe2);},
                    _ => {}
                }
                assembled_vec.push(*offset as u8);
            }
        },
        Op::Op(instruction) => {
            let ins = &instruction[..];
            match ins {
                "nop" => {assembled_vec.push(0x00);},
                "rlc" => {assembled_vec.push(0x07);},
                "rrc" => {assembled_vec.push(0x0f);},
                "ral" => {assembled_vec.push(0x17);},
                "rar" => {assembled_vec.push(0x1f);},
                "rim" => {assembled_vec.push(0x20);},
                "daa" => {assembled_vec.push(0x27);},
                "cma" => {assembled_vec.push(0x2f);},
                "sim" => {assembled_vec.push(0x30);},
                "stc" => {assembled_vec.push(0x37);},
                "cmc" => {assembled_vec.push(0x3f);},
                "rnz" => {assembled_vec.push(0xc0);},
                "rz" => {assembled_vec.push(0xc8);},
                "rnc" => {assembled_vec.push(0xd0);},
                "rc" => {assembled_vec.push(0xd8);},
                "rpo" => {assembled_vec.push(0xe0);},
                "xthl" => {assembled_vec.push(0xe3);},
                "xchg" => {assembled_vec.push(0xeb);},
                "rp" => {assembled_vec.push(0xf0);},
                "di" => {assembled_vec.push(0xf3);},
                "rm" => {assembled_vec.push(0xf8);},
                "sphl" => {assembled_vec.push(0xf9);},
                "ei" => {assembled_vec.push(0xfb);},
                "hlt" => {assembled_vec.push(0x76);},
                _ => {}
            }
        },
        Op::OpRR(instruction,register1, register2 ) => {
            let ins = &instruction[..];
            let reg1 = &register1[..];
            let reg2 = &register2[..];

            match(ins, reg1, reg2) {
                ("mov", "b", x) => {
                    match x {
                        "b" => { assembled_vec.push(0x40); },
                        "c" => { assembled_vec.push(0x41); },
                        "d" => { assembled_vec.push(0x42); },
                        "e" => { assembled_vec.push(0x43); },
                        "h" => { assembled_vec.push(0x44); },
                        "l" => { assembled_vec.push(0x45); },
                        "m" => { assembled_vec.push(0x46); },
                        "a" => { assembled_vec.push(0x47); },
                        _ => { }
                        }
                    },
                ("mov", "c", x) => {
                    match x {
                        "b" => { assembled_vec.push(0x48); },
                        "c" => { assembled_vec.push(0x49); },
                        "d" => { assembled_vec.push(0x4a); },
                        "e" => { assembled_vec.push(0x4b); },
                        "h" => { assembled_vec.push(0x4c); },
                        "l" => { assembled_vec.push(0x4d); },
                        "m" => { assembled_vec.push(0x4e); },
                        "a" => { assembled_vec.push(0x4f); },
                        _ => { }
                        }
                    },
                ("mov", "d", x) => {
                    match x {
                        "b" => { assembled_vec.push(0x50); },
                        "c" => { assembled_vec.push(0x51); },
                        "d" => { assembled_vec.push(0x52); },
                        "e" => { assembled_vec.push(0x53); },
                        "h" => { assembled_vec.push(0x54); },
                        "l" => { assembled_vec.push(0x55); },
                        "m" => { assembled_vec.push(0x56); },
                        "a" => { assembled_vec.push(0x57); },
                        _ => { }
                        }
                },
                ("mov", "e", x) => {
                    match x {
                        "b" => { assembled_vec.push(0x58); },
                        "c" => { assembled_vec.push(0x59); },
                        "d" => { assembled_vec.push(0x5a); },
                        "e" => { assembled_vec.push(0x5b); },
                        "h" => { assembled_vec.push(0x5c); },
                        "l" => { assembled_vec.push(0x5d); },
                        "m" => { assembled_vec.push(0x5e); },
                        "a" => { assembled_vec.push(0x5f); },
                        _ => { }
                        }
                    },
                ("mov", "h", x) => {
                    match x {
                        "b" => { assembled_vec.push(0x60); },
                        "c" => { assembled_vec.push(0x61); },
                        "d" => { assembled_vec.push(0x62); },
                        "e" => { assembled_vec.push(0x63); },
                        "h" => { assembled_vec.push(0x64); },
                        "l" => { assembled_vec.push(0x65); },
                        "m" => { assembled_vec.push(0x66); },
                        "a" => { assembled_vec.push(0x67); },
                        _ => { }
                        }
                    },
                ("mov", "l", x) => {
                    match x {
                        "b" => { assembled_vec.push(0x68); },
                        "c" => { assembled_vec.push(0x69); },
                        "d" => { assembled_vec.push(0x6a); },
                        "e" => { assembled_vec.push(0x6b); },
                        "h" => { assembled_vec.push(0x6c); },
                        "l" => { assembled_vec.push(0x6d); },
                        "m" => { assembled_vec.push(0x6e); },
                        "a" => { assembled_vec.push(0x6f); },
                        _ => { }
                        }
                },
                ("mov", "m", x) => {
                    match x {
                        "b" => { assembled_vec.push(0x70); },
                        "c" => { assembled_vec.push(0x71); },
                        "d" => { assembled_vec.push(0x72); },
                        "e" => { assembled_vec.push(0x73); },
                        "h" => { assembled_vec.push(0x74); },
                        "l" => { assembled_vec.push(0x75); },
                        "a" => { assembled_vec.push(0x77); },
                        _ => { }
                        }
                },
                (_,_,_) => {}
            }
        },
        Op::OpRA(instruction, register, address) => {
            let ins = &instruction[..];
            let register = &register[..];
            let a_u8 = address as u8;

            //for 16 bit
            // TODO: if data is greater than 2^16 then show error 
            let a_lsb = (address & 0x0f) as u8;
            let a_msb = ((address & 0xf0) >> 8) as u8;
            match (ins, register) {
                ("lxi", r) => {
                    match r {
                        "b" => { assembled_vec.push(0x01); },
                        "d" => { assembled_vec.push(0x11); },
                        "h" => { assembled_vec.push(0x21); },
                        "sp" => { assembled_vec.push(0x31); },
                        _ => { }
                    }
                    assembled_vec.push(a_lsb);
                    assembled_vec.push(a_msb);
                },
                ("mvi", r) => {
                    match r {
                        "b" => { assembled_vec.push(0x06); },
                        "c" => { assembled_vec.push(0x0e); },
                        "d" => { assembled_vec.push(0x16); },
                        "e" => { assembled_vec.push(0x1e); },
                        "h" => { assembled_vec.push(0x26); },
                        "l" => { assembled_vec.push(0x2e); },
                        "m" => { assembled_vec.push(0x36); },
                        "a" => { assembled_vec.push(0x3a); },
                        _ => { }
                    }
                    assembled_vec.push(a_u8);
                },
                (_,_) => {

                }
            }
        },
        Op::OpA(instruction, address) => {
            // TODO: remove assembed_vec from inside the matched block, only one is sufficient
            // after match block is completed add assembled_vec.push() 
            let ins = &instruction[..];
            match ins {
                "cpi" => {
                    assembled_vec.push(0xfe);
                    assembled_vec.push(address as u8);
                },
                "ori" => {
                    assembled_vec.push(0xf6);
                    assembled_vec.push(address as u8);
                },
                "ani" => {
                    assembled_vec.push(0xe6);
                    assembled_vec.push(address as u8);
                },
                "sbi" => {
                    assembled_vec.push(0xde);
                    assembled_vec.push(address as u8);
                },
                "in" => {
                    assembled_vec.push(0xdb);
                    assembled_vec.push(address as u8);
                },
                "sui" => {
                    assembled_vec.push(0xd6);
                    assembled_vec.push(address as u8);
                },
                "out" => {
                    assembled_vec.push(0xd3);
                    assembled_vec.push(address as u8);
                },
                "aci" => {
                    assembled_vec.push(0xd8);
                    assembled_vec.push(address as u8);
                },
                "adi" => {
                    assembled_vec.push(0xc6);
                    assembled_vec.push(address as u8);
                },
                "lda" => {
                    assembled_vec.push(0x3a);
                    let add_lsb = (address & 0x0f) as u8;
                    let add_msb = ((address & 0xf0) >> 8) as u8;
                    assembled_vec.push(add_lsb);
                    assembled_vec.push(add_msb);
                },
                "sta" => {
                    assembled_vec.push(0x32);
                    let add_lsb = (address & 0x0f) as u8;
                    let add_msb = ((address & 0xf0) >> 8) as u8;
                    assembled_vec.push(add_lsb);
                    assembled_vec.push(add_msb);
                },
                "jpo" => {
                    assembled_vec.push(0xe2);
                    let add_lsb = (address & 0x0f) as u8;
                    let add_msb = ((address & 0xf0) >> 8) as u8;
                    assembled_vec.push(add_lsb);
                    assembled_vec.push(add_msb);
                },
                "cpo" => {
                    assembled_vec.push(0xe4);
                    let add_lsb = (address & 0x0f) as u8;
                    let add_msb = ((address & 0xf0) >> 8) as u8;
                    assembled_vec.push(add_lsb);
                    assembled_vec.push(add_msb);
                },
                _ => {} }
        },
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

                ("push", a) => {
                    match a {
                        "b" => { assembled_vec.push(0xc5); },
                        "d" => { assembled_vec.push(0xd5); },
                        "h" => { assembled_vec.push(0xe5); },
                        "psw" => { assembled_vec.push(0xf5); },
                        _ => { }
                    }
                },
                ("pop", a) => {
                    match a {
                        "b" => { assembled_vec.push(0xc1); },
                        "d" => { assembled_vec.push(0xd1); },
                        "h" => { assembled_vec.push(0xe1); },
                        "psw" => { assembled_vec.push(0xf1); },
                        _ => { }
                    }
                },
                (_,_) => {}
            }
        }
        _ => {}
    }
}
