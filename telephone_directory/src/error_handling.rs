use std::io::ErrorKind;
use std::fs::File;

pub mod errata {
    pub trait Vocale {
        fn talk(&self) -> String;
    }
}

pub fn longest<'a>(txt: &'a str, txt2 : &'a str) -> &'a str{
    if txt.len() > txt2.len(){
        txt
       }
    else{
        txt2
    }
}




pub fn error_handler(){

let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

   let z = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}