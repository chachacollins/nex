use std::io::{Write, stdin, stdout};
use vm::Vm;
mod compiler;
mod lexer;
mod parser;
mod stack;
mod vm;

use miette::Result;
fn repl() -> Result<()> {
    const GREEN: &str = "\x1b[32m";
    const RED: &str = "\x1b[31m";
    const RESET: &str = "\x1b[0m";
    let mut success = true;
    loop {
        let mut source = String::new();
        if success {
            print!("{}>>{} ", GREEN, RESET);
        } else {
            print!("{}>>{} ", RED, RESET);
        }
        stdout().flush().unwrap();
        match stdin().read_line(&mut source) {
            Ok(_) => match compiler::compile(source.clone()) {
                Ok(chunk) => {
                    let mut vm = Vm::new(source, chunk);
                    match vm.eval() {
                        Ok(result) => {
                            println!("{result}");
                            success = true;
                        }
                        Err(error) => {
                            eprintln!("{:?}", error);
                            success = false;
                        }
                    }
                }
                Err(error) => {
                    eprintln!("{:?}", error);
                    success = false;
                }
            },
            Err(error) => println!("error: {error}"),
        }
    }
}

fn main() -> Result<()> {
    repl()?;
    Ok(())
}
