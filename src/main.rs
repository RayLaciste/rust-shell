#[allow(unused_imports)]
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::Path;

fn main() {
    let mut executables = HashMap::new();
    let path = std::env::var("PATH").unwrap();
    for dir in path.split(':').filter_map(|d| fs::read_dir(d).ok()) {
        for entry in dir {
            let entry = entry.unwrap();
            let file_name = entry.file_name().into_string().unwrap();
            let path = entry.path();
            if let Ok(metadata) = fs::metadata(&path)
                && metadata.is_file()
                && metadata.permissions().mode() & 0o111 != 0
                && !executables.contains_key(&file_name)
            {
                executables.insert(file_name, path);
            }
        }
    }

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading line");

        let input = input.split_whitespace().collect::<Vec<&str>>();
        let command = input[0];
        let arguments: &[&str] = &input[1..];

        let path_env = env::var("PATH").unwrap_or_default();
        let builtin = ["echo", "type", "exit", "pwd"];

        match command {
            "echo" => {
                println!("{}", arguments.join(" "));
            }
            // ----- EXIT COMMAND
            "exit" => {
                break;
            }
            // ----- PWD COMMAND
            "pwd" => {
                println!("{}", env::current_dir().unwrap().display());
            }
            "type" => match arguments[0] {
                arg @ ("echo" | "exit" | "pwd" | "type") => println!("{} is a shell builtin", arg),
                arg => {
                    if let Some(path) = executables.get(arg) {
                        println!("{} is {}", arg, path.to_str().to_owned().unwrap());
                    } else {
                        println!("{}: not found", arg);
                    }
                }
            },
            command => {
                if let Some(path) = executables.get(command) {
                    let mut process = std::process::Command::new(path)
                        .arg0(command)
                        .args(arguments)
                        .spawn()
                        .unwrap();
                    process.wait().unwrap();
                } else {
                    println!("{}: command not found", command);
                }
            }
        }
    }
}
