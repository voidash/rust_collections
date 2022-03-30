pub mod ast;
pub mod utils;
pub mod assembler;

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
}

fn main() {
    println!("this is assembly");
}
