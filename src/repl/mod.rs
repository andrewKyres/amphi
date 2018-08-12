use std;
use std::io;
use std::io::Write;
use vm::VM;

/// Core structure for the REPL for the Assembler
pub struct REPL {
    /// For tracking a user's history of commands
    command_buffer: Vec<String>,
    /// The VM the REPL will use to execute code
    vm: VM,
}

impl REPL {
    /// Creates and returns a new assembly REPL
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![],
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to amphi!");
        loop {
            // TODO: Create this outside of the loop and re-use it every iteration
            let mut buffer = String::new();

            
            let stdin = io::stdin();

            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");


            stdin.read_line(&mut buffer).expect("Unable to read line from user");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());
            match buffer {
                ".quit" => {
                    println!("Bye, have a beautiful day!");
                    std::process::exit(0);
                },
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                },
                _ => {
                    println!("Invalid input");
                },
            }
        }
    }
}
