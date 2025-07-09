use miette::Result;
use std::fs;
use std::io::{Write, stdin, stdout};
use vm::Vm;
mod compiler;
mod lexer;
mod parser;
mod stack;
mod vm;

struct Repl {
    history: Vec<String>,
    success: bool,
    input: String,
}

impl Repl {
    fn new() -> Self {
        Self {
            history: Vec::new(),
            success: true,
            input: String::new(),
        }
    }
    fn run(&mut self) -> Result<()> {
        const GREEN: &str = "\x1b[32m";
        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";
        self.success = true;
        loop {
            self.input.clear();
            if self.success {
                print!("{}>>{} ", GREEN, RESET);
            } else {
                print!("{}>>{} ", RED, RESET);
            }
            stdout().flush().expect("Failed to flush std out");
            stdin()
                .read_line(&mut self.input)
                .expect("Failed to read line");

            let input = self.input.split_whitespace().collect::<Vec<&str>>();
            if let Some(&"history") = input.first() {
                match input.get(1).copied() {
                    Some("write") => {
                        fs::write("history.txt", self.history.join("\n"))
                            .expect("Could not write to history.txt");
                        println!("History written to file.");
                        self.success = true;
                    }
                    Some("load") => {
                        if let Ok(contents) = fs::read_to_string("history.txt") {
                            self.history = contents.lines().map(String::from).collect();
                            println!("History loaded from file.");
                            self.success = true;
                        } else {
                            eprintln!("Could not read history.txt");
                        }
                    }
                    Some(_) => {
                        eprintln!("Unknown history subcommand.");
                    }
                    _ => {
                        println!("---------HISTORY-----------");
                        for (i, history) in self.history.iter().enumerate() {
                            println!("{}: {}", i + 1, history);
                        }
                        self.success = true;
                        println!("-----------------------------");
                    }
                }
            } else if let Some(&"quit") = input.first() {
                return Ok(());
            } else {
                match compiler::compile(&self.input) {
                    Ok(chunk) => {
                        let mut vm = Vm::new(&self.input, chunk);
                        match vm.eval() {
                            Ok(result) => {
                                println!("{result}");
                                self.success = true;
                                self.history.push(format!(
                                    "{} = {}",
                                    self.input.split('\n').nth(0).unwrap(),
                                    result
                                ));
                            }
                            Err(error) => {
                                eprintln!("{:?}", error);
                                self.success = false;
                            }
                        }
                    }
                    Err(error) => {
                        eprintln!("{:?}", error);
                        self.success = false;
                    }
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let mut repl = Repl::new();
    repl.run()?;
    Ok(())
}
