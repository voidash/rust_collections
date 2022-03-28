use std::collections::HashMap;
use super::Evaluable;


impl Evaluable for i32 {
    fn evaluate(&self,_var_map: &mut HashMap<String, i32>) -> i32 {
        *self
    }
}
