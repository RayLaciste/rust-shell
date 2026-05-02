#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::path::Path;

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
                /*
                * Gets the value of environment variable PATH
                *  result is either a string or defaults to ""
                */
                let path_env = env::var("PATH").unwrap_or_default();
                let found = path_env.split(':').find_map(|dir| {
                    let full_path = format!("{}/{}", dir, args);
                    if Path::new(&full_path).is_file() {
                        Some(full_path)
                    } else {
                        None
                    }
                });
                match found {
                    Some(path) => println!("{} is {}", args, path),
                    None => println!("{}: not found", args),
                }
            }
        } else {
            println!("{}: command not found", command);
        }
    }
}
