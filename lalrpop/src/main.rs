
pub mod ast;
pub mod utils;
pub mod assembler;

use std::collections::HashMap;

use utils::convert_8085_hex_to_i32;

use crate::assembler::assemble;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub assembly);

#[test]
fn literal_check() {
    assert_eq!(assembly::DecParser::new().parse("12"), Ok(12));
}

#[test]
fn register_check() {
    assert_eq!(assembly::RegisterParser::new().parse("a").unwrap(), "a");
    assert_ne!(assembly::RegisterParser::new().parse("Z"), Ok("Z"));
}

#[test]
fn instruction_check() {
    assert_eq!(
        assembly::InstructionParser::new().parse("mov").unwrap(),
        "mov"
    );
    assert_ne!(
        assembly::InstructionParser::new().parse("ldaxx"),
        Ok("ldaxx")
    );
}

#[test]
fn opcode_check() {
    println!(
        "{:?}",
        assembly::OpcodeParser::new()
            .parse("check: mov a    , b")
            .unwrap()
    );
    println!(
        "{:?}",
        assembly::OpcodeParser::new().parse("mvi a, 32D").unwrap()
    );
    println!(
        "{:?}",
        assembly::OpcodeParser::new().parse("mvi a").unwrap()
    );

    let label_offset_map: HashMap<String, u32> = HashMap::new();
    println!("{:?}",assemble(assembly::OpcodeParser::new().parse("mvi a, 32D").unwrap(), &label_offset_map));
    println!("{:?}",assemble(assembly::OpcodeParser::new().parse("add b").unwrap(), &label_offset_map));
}

fn remove_comments(line: &str) -> &str{
    if let Some(index) = line.find(";") {
        return &line[..index];
    } 
    line
}

fn fix_hexadecimal(line: &str) -> String {
    if line.ends_with("h") {
        //find comma
        if let Some(decimal_start_pos) = line.rfind(" ") {
             let d: i32 = convert_8085_hex_to_i32(&line[decimal_start_pos..line.len()].trim());
             let ret_val: String = format!("{} {}", &line[..decimal_start_pos], i32::to_string(&d));
             return ret_val;
        }
    }
    return line.to_string();
}

#[test] 
fn remove_comments_test() {
    let data = "mov a, b ; where all the birds fly everything seems merrier";
    let b = ";where all the birds fly everything seems merrier";
    println!("{}", remove_comments(data));
    println!("{}", remove_comments(b));
}

#[test]
fn test_fix_hexadecimal() {
    let data = "mvi a, 12h";
    let y = "ldax ffffh";
    println!("{}", fix_hexadecimal(data));
    println!("{}", fix_hexadecimal(y));
}

#[allow(unused_variables,unused_mut)]
fn main() {

    let mut label_offset_map: HashMap<String, u32> = HashMap::new();

    let lines = vec![ 
        "nop; genie niskyo",
        "test: mvi a, 45h     ",
        "lda 3432h",
        "jmp test"
    ];

    let mut assembly_code :Vec<u8> = Vec::new(); 
    let mut line_number: u32 = 0;
    for line in lines {
       line_number += 1; 
       let line = &fix_hexadecimal(remove_comments(line.trim()))[..];
       match assembly::OpcodeParser::new().parse(line) {
           Ok(ops) => {
               if let Some(l) = &ops.Label {
                   label_offset_map.insert(l.clone(), line_number);
               }
                assembly_code.append(&mut assemble(ops, &label_offset_map));
           },
            Err(_) => {}
       }
       println!("{}", line);
    }

   println!("{:?}", assembly_code);

}
