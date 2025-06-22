use crate::lexer::{Lexer, TokenKind};
use crate::parser;
use crate::vm::{Chunk, Opcode};
use miette::Result;

fn traverse_and_compile(nodes: parser::Nodes, chunk: &mut Chunk) {
    use parser::Nodes::*;
    match nodes {
        Number(offset, number) => chunk.push(Opcode::Num(offset, number)),
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

pub fn compile(source: String) -> Result<Chunk> {
    let lexer = Lexer::new(&source);
    let ast = parser::parse(lexer.source, &mut lexer.peekable(), 0)?;
    let mut chunk = Chunk::new();
    traverse_and_compile(ast, &mut chunk);
    chunk.push(Opcode::Ret);
    Ok(chunk)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Vm;

    #[test]
    fn test_compiler() {
        let source = "(1 + 2) * 3";
        let chunk = compile(source.to_string()).unwrap();
        let mut vm = Vm::new(source.to_string(), chunk);
        let result = vm.eval().unwrap();
        assert_eq!(result, "9");
    }
}
