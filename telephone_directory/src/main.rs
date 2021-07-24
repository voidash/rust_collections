// pub mod t_struct;
pub mod error_handling; 
 
use error_handling::error_handler;
use error_handling::longest;

use std::io;

#[allow(dead_code)]
fn take_input(prompt: &str, data : &mut String){
    println!("{}",String::from(prompt));
    io::stdin()
        .read_line(data)
        .expect("Couldn't read it");
}

fn main() {
    {
    let m1 = String::from("Hallelulah");
    let long; 
    {
        let m2 = String::from("Hallelulah");
        long = longest(&m1[..], &m2[..]);
    }
    println!("{}",long);
    }
    // error_handler();
    
    // let mut name = String::new();
    // let mut address = String::new();
    // let mut number = String::new();
   
    // take_input("Enter your name",&mut name);
    // take_input("Enter your address",&mut address);
    // take_input("Enter your number",&mut number);

    // let number:usize = number.trim().parse().expect("was expecting a number in number field"); 
        
    // let mut data = t_struct::Directory{
    //     name : name,
    //     number : number,
    //     address: address,
    //     t_type: t_struct::TelephoneType::Default 
    // };


    // data.set_name("piko");
    // data.set_t_type();

    // println!("{:#?}",data);



}
