//usr/bin/true; rustc -o /tmp/$0.bin $0; /tmp/$0.bin

// TODO
// Add Redirection operators : < , << ,  > , >> 

use std::{io::{stdin, stdout, Write}, process::{Command, Stdio}, env};
use std::path::Path;

fn main() {
    loop {
        print!("{} >", env::current_dir().unwrap().display());
        stdout().flush().unwrap();


        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split("|").peekable();
        let mut previous_command : Option<std::process::Child> = None;


    while let Some(command) = commands.next() {
        println!("{}", command);
        let mut parts = command.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/",|x| *x);
                let root =  Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }

                previous_command = None;
            },
            "exit" => return,

            command => {
            let stdin = previous_command.map_or(Stdio::inherit(),|output: std::process::Child| {
                Stdio::from(output.stdout.unwrap())
            });

            let stdout = if commands.peek().is_some() {
                Stdio::piped()
            } else {
                Stdio::inherit()
            };

            let output = Command::new(command).args(args).stdin(stdin).stdout(stdout).spawn();
            match output {
                Ok(output) => {
                    previous_command = Some(output);
                }
                Err(e) => {
                    previous_command = None;
                    eprintln!("{}", e);
                }
                };
            }

        }
    }

    if let Some(mut final_command) = previous_command {
        final_command.wait().unwrap();
    }

}

}
