use std::env;
use std::process;

use grepper::Config;
use grepper::run;

fn main() {
    let args : Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| { 
        eprintln!("problem parsing arguments : {} ", err);
        process::exit(1);
    }); 

    println!("the string is {} and filename is {}",config.query, config.filename);
    
    if let Err(e) = run(config) {
        eprintln!("application error: {}",e);
        process::exit(1);
    }
    
   }


