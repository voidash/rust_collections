use std::collections::HashMap;
use std::string::String;
use super::Evaluable;
use super::AstData;


pub struct Variable {
    pub identifier: String,
    pub data: Option<Box<AstData>>
}

impl Variable {
    pub fn new_boxed(identifier: String, data: Box<AstData>) -> Variable {
        let data_option =Some(data);
        Variable {
            identifier,
            data: data_option
        }
    }

    pub fn new_read(identifier: String) -> Variable {
        Variable {
            identifier,
            data: None
        } 
    }
}

impl Evaluable for Variable {
    fn evaluate (&self, var_map: &mut HashMap<String,i32>) -> i32 {
        match &self.data {
            Some(data) => (*data).evaluate(var_map),
            None => match var_map.get(&self.identifier) {
                Some(val) => {
                    let value = *val;
                    value.evaluate(var_map)
                },
                None => panic!("variable {} not found", &self.identifier)
            }
        }
    }
}


