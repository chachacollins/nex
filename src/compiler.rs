use crate::lexer::{Lexer, TokenKind};
use crate::parser;
use crate::vm::{Chunk, Opcode, Vm};

#[derive(Debug)]
pub enum CompilerError {
    ParserError(parser::ParserError),
}

fn traverse_and_compile(nodes: parser::Nodes, chunk: &mut Chunk) {
    use parser::Nodes::*;
    match nodes {
        Number(number) => chunk.push(Opcode::Num(number)),
        Operator(op_node) => {
            if let Some(node) = op_node.left {
                traverse_and_compile(*node, chunk);
            }
            if let Some(node) = op_node.right {
                traverse_and_compile(*node, chunk)
            }
            match op_node.op.kind {
                TokenKind::Plus => chunk.push(Opcode::Add),
                TokenKind::Minus => chunk.push(Opcode::Sub),
                TokenKind::Div => chunk.push(Opcode::Div),
                TokenKind::Mult => chunk.push(Opcode::Mult),
                TokenKind::Mod => chunk.push(Opcode::Mod),
                _ => unreachable!(),
            }
        }
    }
}

pub fn compile(source: String) -> Result<Chunk, CompilerError> {
    let mut lexer = Lexer::new(&source);
    let ast = parser::parse(&mut lexer.peekable(), 0);
    match ast {
        Ok(nodes) => {
            let mut chunk = Chunk::new();
            traverse_and_compile(nodes, &mut chunk);
            chunk.push(Opcode::Ret);
            Ok(chunk)
        }
        Err(parser_error) => Err(CompilerError::ParserError(parser_error)),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compiler() {
        let source = "(1 + 2) * 3";
        let chunk = compile(source.to_string()).unwrap();
        let mut vm = Vm::new(chunk);
        let result = vm.eval().unwrap();
        assert_eq!(result, "9");
    }
}
