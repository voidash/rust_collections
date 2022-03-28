use std::collections::HashMap;


pub mod variable;
mod evaluable_int;

pub type AstData = dyn Evaluable;


pub trait Evaluable {
    fn evaluate (&self, var_map: &mut HashMap<String, i32> ) -> i32;
}

pub enum Operation {
    ADD,
    SUB, 
    MUL, 
    DIV
}

pub struct Ast {
    pub left: Box::<AstData>,
    pub op: Operation,
    pub right: Box::<AstData>
}

impl Ast {
    pub fn new(left: Box<AstData> , op:Operation, right: Box<AstData>) -> Ast {
        Ast {
            left, op, right
        }
    }
}

impl Evaluable for Ast {
    fn evaluate(&self,var_map: &mut HashMap<String, i32>)   -> i32 {
        let left = (*self.left).evaluate(var_map);
        let right= (*self.right).evaluate(var_map);

    match self.op {
        Operation::ADD => left + right,
        Operation::SUB => left - right,
        Operation::MUL => left * right,
        Operation::DIV => left / right
    }
    }
}

