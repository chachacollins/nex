use crate::stack::Stack;
use miette::{Diagnostic, Result, SourceSpan};
use std::fmt::Write;
use thiserror::Error;

pub enum Opcode {
    Add,
    Sub,
    Div,
    Mult,
    Mod,
    Num(u8, f64),
    Ret,
}

pub type Chunk = Vec<Opcode>;

pub struct Vm {
    chunk: Chunk,
    stack: Stack,
    ip: usize,
    src: String,
}

#[derive(Error, Debug, Diagnostic)]
#[error("No return opcode emitted!")]
struct NoReturnOpcode {}

#[derive(Error, Debug, Diagnostic)]
#[error("Division by zero!")]
#[diagnostic(help("try to divide by anything other than that"))]
struct DivByZero {
    #[source_code]
    src: String,
    #[label("This part here")]
    bad_bit: SourceSpan,
}

macro_rules! binary_op {
    ($self: expr, $op:tt) => {{
        let (offset_b, b) = $self.stack.pop()?;
        let (offset_a, a)= $self.stack.pop()?;
        if stringify!($op) == "/" {
            if b == 0. {
                let span = offset_b - offset_a;
                let a_len = (a.log(10.0) + 1.0).abs() as u8;
                return Err(DivByZero {
                    src: $self.src.to_string(),
                    bad_bit: ((offset_a - a_len) as usize, (span + a_len) as usize).into()
                })?
            }
        }
        $self.stack.push((0, a $op b))?;
    }};
}

impl Vm {
    pub fn new(source: String, chunk: Chunk) -> Self {
        assert!(chunk.len() > 0);
        Self {
            chunk,
            stack: Stack::new(),
            ip: 0,
            src: source,
        }
    }
    pub fn eval(&mut self) -> Result<String> {
        let mut result = String::new();
        use Opcode::*;
        if self.ip >= self.chunk.len() {
            return Err(NoReturnOpcode {})?;
        }
        loop {
            let instruction = &self.chunk[self.ip];
            self.ip += 1;
            match instruction {
                Add => binary_op!(self, +),
                Sub => binary_op!(self, -),
                Mult => binary_op!(self, *),
                Mod => binary_op!(self, %),
                Div => binary_op!(self, /),
                Num(offset, num) => {
                    self.stack.push((*offset, *num))?;
                }
                Ret => {
                    let (_, ret) = self.stack.pop()?;
                    write!(&mut result, "{}", ret).expect("Failed to write to result buffer");
                    break;
                }
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! define_op {
        ($chunk: expr, $op_code: expr) => {
            $chunk.push(Opcode::Num(0, 20.));
            $chunk.push(Opcode::Num(1, 10.));
            $chunk.push($op_code);
            $chunk.push(Opcode::Ret);
        };
    }

    #[test]
    fn test_vm_add() {
        let mut chunk = Chunk::new();
        define_op!(chunk, Opcode::Add);
        let mut vm = Vm::new("20 + 10".to_string(), chunk);
        let result = vm.eval().unwrap();
        assert_eq!(result, "30");
    }

    #[test]
    fn test_vm_sub() {
        let mut chunk = Chunk::new();
        define_op!(chunk, Opcode::Sub);
        let mut vm = Vm::new("20 - 10".to_string(), chunk);
        let result = vm.eval().unwrap();
        assert_eq!(result, "10");
    }

    #[test]
    fn test_vm_div() {
        let mut chunk = Chunk::new();
        define_op!(chunk, Opcode::Div);
        let mut vm = Vm::new("20 / 10".to_string(), chunk);
        let result = vm.eval().unwrap();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_vm_mult() {
        let mut chunk = Chunk::new();
        define_op!(chunk, Opcode::Mult);
        let mut vm = Vm::new("20 * 10".to_string(), chunk);
        let result = vm.eval().unwrap();
        assert_eq!(result, "200");
    }

    #[test]
    fn test_vm_mod() {
        let mut chunk = Chunk::new();
        define_op!(chunk, Opcode::Mod);
        let mut vm = Vm::new("20 % 10".to_string(), chunk);
        let result = vm.eval().unwrap();
        assert_eq!(result, "0");
    }
}
