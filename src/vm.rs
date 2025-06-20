use crate::stack::{Stack, Value};
use std::io::Write;

enum Opcode {
    Add,
    Sub,
    Div,
    Mult,
    Mod,
    Num(f64),
    Ret,
}

type Chunk = Vec<Opcode>;

struct Vm {
    chunk: Chunk,
    stack: Stack,
    ip: usize,
}

#[derive(Debug)]
enum VmError {
    OverflowChunk,
    StackError,
}

fn write_result<W: Write>(writer: &mut W, value: Value) -> std::io::Result<()> {
    writeln!(writer, "{}", value)
}

fn print_result(value: Value) {
    write_result(&mut std::io::stdout(), value).unwrap();
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
    fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            stack: Stack::new(),
            ip: 0,
        }
    }
    fn run(&mut self) -> Result<(), VmError> {
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
                    print_result(self.stack.pop().unwrap());
                    break;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vm() {
        let mut chunk = Chunk::new();
        chunk.push(Opcode::Num(10.));
        chunk.push(Opcode::Num(10.1));
        chunk.push(Opcode::Add);
        chunk.push(Opcode::Ret);
        let mut vm = Vm::new(chunk);
        vm.run().unwrap();
    }
}
