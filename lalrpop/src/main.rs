#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub calculator1);


#[test]
fn calculator1() {
    assert!(calculator1::TermParser::new().parse("22").is_ok());
}


lalrpop_mod!(pub calculator2);

#[test]
fn calculator2() {
    assert!(calculator2::TermParser::new().parse("4+3").is_ok());
}

fn main() {
    println!("Hello, world!");
}
