#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {

        print!("$ ");
        io::stdout().flush().unwrap();
 
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();

        if command == "exit" {
            break;
        } else if command.starts_with("echo ") {
            println!("{}", &command[5..]);
        } else if command.starts_with("type ") { 
            let args = &command[5..];
            if args == "echo" || args == "type" || args == "exit" {
                println!("{} is a shell builtin", args);
            } else {
                println!("{}: not found", args);
            }
        } else {
            println!("{}: command not found", command);
        }
    }
}
