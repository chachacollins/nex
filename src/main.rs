use miette::Result;
use std::io::{Write, stdin, stdout};
use vm::Vm;
mod compiler;
mod lexer;
mod parser;
mod stack;
mod vm;

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
        stdout().flush().expect(
            "This should never fail and if it does there is no point in continuing the application",
        );
        stdin().read_line(&mut source).expect(
            "This should normally never fail and if it does there is no point in continuing the application",
        );
        match compiler::compile(&source) {
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
        }
    }
}

fn main() -> Result<()> {
    repl()?;
    Ok(())
}
