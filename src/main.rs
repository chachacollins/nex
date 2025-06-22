use std::io::{Write, stdin, stdout};
use vm::Vm;
mod compiler;
mod lexer;
mod parser;
mod stack;
mod vm;

fn repl() -> ! {
    loop {
        let mut source = String::new();
        print!(">> ");
        stdout().flush().unwrap();
        match stdin().read_line(&mut source) {
            Ok(_) => {
                let chunk = compiler::compile(source).unwrap();
                let mut vm = Vm::new(chunk);
                let result = vm.eval().unwrap();
                println!("{result}")
            }
            Err(error) => println!("error: {error}"),
        }
    }
}

fn main() {
    repl();
}
