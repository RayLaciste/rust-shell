use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::Path;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command: String = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command: &str = command.trim(); // command + args

        let path_env = env::var("PATH").unwrap_or_default();
        let builtin = ["echo", "type", "exit"];

        // ----- EXIT COMMAND
        if command == "exit" {
            break;
        }

        // ----- ECHO COMMAND
        if let Some(rest) = command.strip_prefix("echo ") {
            println!("{}", rest);
            continue;
        }

        // ----- TYPE COMMANAND
        if let Some(rest) = command.strip_prefix("type ") {
            let args: &str = rest;
            if builtin.contains(&args) {
                println!("{} is a shell builtin", args);
            } else {
                let found = path_env.split(':').find_map(|dir| {
                    let full_path = format!("{}/{}", dir, args);
                    if Path::new(&full_path).is_file() {
                        // does the file exist
                        let permissions = std::fs::metadata(&full_path).unwrap().permissions();
                        // mode gets the permission bits
                        // 0o111 in octal is 001 001 001
                        // ( r w x | r w x | r w x) read write execute
                        if permissions.mode() & 0o111 != 0 {
                            // is the file executable
                            Some(full_path)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                });
                match found {
                    Some(path) => println!("{} is {}", args, path),
                    None => println!("{}: not found", args),
                }
            }

            continue;
        }

        // ----- EXECUTE PROGRAM
        let parts: Vec<&str> = command.split_whitespace().collect();
        if let Some((&program, args)) = parts.split_first() {
            let found = path_env.split(':').find_map(|dir| {
                let full_path = format!("{}/{}", dir, program);
                if Path::new(&full_path).is_file() {
                    let permissions = std::fs::metadata(&full_path).unwrap().permissions();
                    if permissions.mode() & 0o111 != 0 {
                        Some(full_path)
                    } else {
                        None
                    }
                } else {
                    None
                }
            });

            if let Some(path) = found {
                std::process::Command::new(&path)
                    .arg0(program)
                    .args(args)
                    .status()
                    .unwrap();
                continue;
            }
        }

        // ----- default
        println!("{}: command not found", command);
    }
}
