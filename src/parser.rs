use crate::lexer::{Lexer, Token, TokenKind};
use std::fmt;
use std::iter::Peekable;

trait Nodes: fmt::Display {}

struct OperatorNode<T: Nodes> {
    op: Token,
    left: Box<T>,
    right: Box<T>,
}

impl<T: Nodes> Nodes for OperatorNode<T> {}

impl<T: Nodes> fmt::Display for OperatorNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.op.kind {
            TokenKind::Plus => write!(f, "(+ {} {})", self.left, self.right),
            TokenKind::Minus => write!(f, "(- {} {})", self.left, self.right),
            TokenKind::Div => write!(f, "(/ {} {})", self.left, self.right),
            TokenKind::Mult => write!(f, "(* {} {})", self.left, self.right),
            TokenKind::Mod => write!(f, "(% {} {})", self.left, self.right),
            _ => unreachable!(),
        }
    }
}

enum Node {
    Number(Token),
    Operator(OperatorNode<Node>),
}

impl Nodes for Node {}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Number(t) => match &t.kind {
                TokenKind::Num(x) => write!(f, "{}", x),
                _ => unreachable!(),
            },
            Node::Operator(op) => op.fmt(f),
        }
    }
}

#[derive(Debug)]
enum ParseError {
    UnexpectedToken,
    UnexpectedEof,
}

fn get_precedence(kind: &TokenKind) -> u8 {
    use TokenKind::*;
    match kind {
        Plus | Minus | Mod => 1,
        Mult | Div => 2,
        _ => unreachable!(),
    }
}

fn parse(lexer: &mut Peekable<Lexer>, prev_precedence: u8) -> Result<Node, ParseError> {
    use TokenKind::*;
    let token = lexer.next().ok_or(ParseError::UnexpectedEof)?;
    let mut lhs = match token.kind {
        Num(_) => Node::Number(token),
        _ => return Err(ParseError::UnexpectedToken),
    };
    loop {
        if let Some(next_token) = lexer.peek() {
            match next_token.kind {
                Plus | Minus | Div | Mod | Mult => {
                    let precedence = get_precedence(&next_token.kind);
                    if precedence > prev_precedence {
                        let consumed_token = lexer.next().unwrap();
                        let right_node = parse(lexer, precedence)?;
                        let op_node = OperatorNode {
                            op: consumed_token,
                            left: Box::new(lhs),
                            right: Box::new(right_node),
                        };
                        lhs = Node::Operator(op_node);
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        } else {
            break;
        };
    }
    Ok(lhs)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_int() {
        let source = "1";
        let lexer = Lexer::new(&source);
        let parsed: Node = parse(&mut lexer.peekable(), 0).unwrap();
        assert_eq!(parsed.to_string(), "1");
    }

    #[test]
    fn parse_expr() {
        let source = "3 * 2 + 1";
        let lexer = Lexer::new(&source);
        let parsed: Node = parse(&mut lexer.peekable(), 0).unwrap();
        assert_eq!(parsed.to_string(), "(+ (* 3 2) 1)");
    }
}
