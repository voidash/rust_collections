#[macro_use] 
extern crate lalrpop_util;

lalrpop_mod!(pub calculator1);


#[test]
fn calculator1() {
    assert!(calculator1::TermParser::new().parse("22").is_ok());
    assert!(calculator1::TermParser::new().parse("(22)").is_ok());
    assert!(calculator1::TermParser::new().parse("((22)").is_err());
}

lalrpop_mod!(pub calculator2);

#[test]
fn calculator2() {
    assert!(calculator2::ExprParser::new().parse("22*34+12").is_ok());
    assert_eq!(calculator2::ExprParser::new().parse("22*34+12").unwrap(), 22*34+12);
}

lalrpop_mod!(pub calculator4);

pub mod ast;

#[test]
fn calculator4() {
    let expr = calculator4::ExprParser::new().parse("22*44+23").unwrap();
    println!("{:?}",expr);
}


lalrpop_mod!(pub assembly);
pub mod assembly_ast;
#[test]
fn check_assembly() {
    let expr = assembly::ExpressionParser::new().parse("hello: MOV a, b");
    println!("{:?}", expr);
}

fn main() {
    println!("test");
}

//label: opcode ;comment
//:b1
