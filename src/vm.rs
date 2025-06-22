use crate::stack::{Stack, Value};
use std::fmt::Write;

pub enum Opcode {
    Add,
    Sub,
    Div,
    Mult,
    Mod,
    Num(f64),
    Ret,
}

pub type Chunk = Vec<Opcode>;

pub struct Vm {
    chunk: Chunk,
    stack: Stack,
    ip: usize,
}

#[derive(Debug)]
pub enum VmError {
    OverflowChunk,
    StackError,
}

//TODO: Fix error handling
macro_rules! binary_op {
    ($self: expr, $op:tt) => {{
        let b = $self.stack.pop().unwrap();
        let a = $self.stack.pop().unwrap();
        // if !a.is_number() || !b.is_number() {
        //     eprintln!("Trying to  {:?} { }{:?} which are of different types",stringify!(op), a, b);
        //     return VmError::DifferentTypes;
        // }
        $self.stack.push(a $op b).unwrap();
    }};
}

//TODO: proper error handling
impl Vm {
    pub fn new(chunk: Chunk) -> Self {
        assert!(chunk.len() > 0);
        Self {
            chunk,
            stack: Stack::new(),
            ip: 0,
        }
    }
    pub fn eval(&mut self) -> Result<String, VmError> {
        let mut result = String::new();
        use Opcode::*;
        if self.ip >= self.chunk.len() {
            return Err(VmError::OverflowChunk);
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
                Num(x) => {
                    self.stack.push(*x).unwrap();
                }
                Ret => {
                    write!(&mut result, "{}", self.stack.pop().unwrap())
                        .expect("Failed to write to result buffer");
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
            $chunk.push(Opcode::Num(20.));
            $chunk.push(Opcode::Num(10.));
            $chunk.push($op_code);
            $chunk.push(Opcode::Ret);
        };
    }

    #[test]
    fn test_vm_add() {
        let mut chunk = Chunk::new();
        define_op!(chunk, Opcode::Add);
        let mut vm = Vm::new(chunk);
        let result = vm.eval().unwrap();
        assert_eq!(result, "30");
    }

    #[test]
    fn test_vm_sub() {
        let mut chunk = Chunk::new();
        define_op!(chunk, Opcode::Sub);
        let mut vm = Vm::new(chunk);
        let result = vm.eval().unwrap();
        assert_eq!(result, "10");
    }

    #[test]
    fn test_vm_div() {
        let mut chunk = Chunk::new();
        define_op!(chunk, Opcode::Div);
        let mut vm = Vm::new(chunk);
        let result = vm.eval().unwrap();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_vm_mult() {
        let mut chunk = Chunk::new();
        define_op!(chunk, Opcode::Mult);
        let mut vm = Vm::new(chunk);
        let result = vm.eval().unwrap();
        assert_eq!(result, "200");
    }

    #[test]
    fn test_vm_mod() {
        let mut chunk = Chunk::new();
        define_op!(chunk, Opcode::Mod);
        let mut vm = Vm::new(chunk);
        let result = vm.eval().unwrap();
        assert_eq!(result, "0");
    }
}
