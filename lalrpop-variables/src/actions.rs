use lazy_static::lazy_static;

use std::collections::HashMap;
use std::string::String;
use std::sync::Mutex;


lazy_static!(
    static ref VARIABLES: Mutex<HashMap<String, i32>> = Mutex::new(HashMap::new());
);

#[allow(dead_code)]
pub fn set(id:String, value: i32) -> i32 {
    let mut vars = VARIABLES.lock().unwrap();
    (*vars).insert(id, value);
    value
}


#[allow(dead_code)]
pub fn get(id: String) -> i32 {
    let vars = VARIABLES.lock().unwrap();
    match (*vars).get(&id) {
        Some(_val) => *_val,
        None => {
            println!("Variable {} not found! Initializing to 0",&id);
            0
        }
    }
}

#[allow(dead_code)]
pub fn print(n: i32) {
    println!("{}",n);
}
