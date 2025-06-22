use std::io::{Write, stdin, stdout};
use vm::Vm;
mod compiler;
mod lexer;
mod parser;
mod stack;
mod vm;

use miette::Result;
fn repl() -> Result<()> {
    loop {
        let mut source = String::new();
        print!(">> ");
        stdout().flush().unwrap();
        match stdin().read_line(&mut source) {
            Ok(_) => {
                let chunk = compiler::compile(source.clone())?;
                let mut vm = Vm::new(source, chunk);
                let result = vm.eval()?;
                println!("{result}")
            }
            Err(error) => println!("error: {error}"),
        }
    }
}

fn main() -> Result<()> {
    repl()?;
    Ok(())
}
