use crate::lexer::{Lexer, Token, TokenKind};
use std::fmt;
use std::iter::Peekable;
use std::str::FromStr;

pub trait Node: fmt::Display {}

pub struct OperatorNode<T: Node> {
    pub op: Token,
    pub left: Option<Box<T>>,
    pub right: Option<Box<T>>,
}

impl<T: Node> Node for OperatorNode<T> {}

impl<T: Node> fmt::Display for OperatorNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.op.kind {
            TokenKind::Plus => write!(
                f,
                "(+ {} {})",
                self.left.as_ref().unwrap(),
                self.right.as_ref().unwrap()
            ),
            TokenKind::Minus => write!(
                f,
                "(- {} {})",
                self.left.as_ref().unwrap(),
                self.right.as_ref().unwrap()
            ),
            TokenKind::Div => write!(
                f,
                "(/ {} {})",
                self.left.as_ref().unwrap(),
                self.right.as_ref().unwrap()
            ),
            TokenKind::Mult => write!(
                f,
                "(* {} {})",
                self.left.as_ref().unwrap(),
                self.right.as_ref().unwrap()
            ),
            TokenKind::Mod => write!(
                f,
                "(% {} {})",
                self.left.as_ref().unwrap(),
                self.right.as_ref().unwrap()
            ),
            _ => unreachable!(),
        }
    }
}

pub enum Nodes {
    Number(f64),
    Operator(OperatorNode<Nodes>),
}

impl Node for Nodes {}
impl fmt::Display for Nodes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Nodes::Number(num) => write!(f, "{}", num),
            Nodes::Operator(op) => op.fmt(f),
        }
    }
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken,
    UnexpectedEof,
    SyntaxError,
}

fn get_precedence(kind: &TokenKind) -> u8 {
    use TokenKind::*;
    match kind {
        Plus | Minus | Mod => 1,
        Mult | Div => 2,
        _ => unreachable!(),
    }
}

pub fn parse(lexer: &mut Peekable<Lexer>, prev_precedence: u8) -> Result<Nodes, ParserError> {
    use TokenKind::*;
    let token = lexer.next().ok_or(ParserError::UnexpectedEof)?;
    let mut lhs = match token.kind {
        Num(num) => Nodes::Number(f64::from_str(&num).map_err(|_| ParserError::SyntaxError)?),
        Lparen => {
            let expression = parse(lexer, 0)?;
            let consumed = lexer.next().ok_or(ParserError::SyntaxError)?;
            if consumed.kind != Rparen {
                return Err(ParserError::SyntaxError);
            }
            expression
        }
        _ => return Err(ParserError::UnexpectedToken),
    };
    loop {
        if let Some(next_token) = lexer.peek() {
            match next_token.kind {
                Plus | Minus | Div | Mod | Mult => {
                    let precedence = get_precedence(&next_token.kind);
                    if precedence <= prev_precedence {
                        break;
                    } else {
                        let consumed_token = lexer.next().unwrap();
                        let right_node = parse(lexer, precedence)?;
                        let op_node = OperatorNode {
                            op: consumed_token,
                            left: Some(Box::new(lhs)),
                            right: Some(Box::new(right_node)),
                        };
                        lhs = Nodes::Operator(op_node);
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
        let parsed: Nodes = parse(&mut lexer.peekable(), 0).unwrap();
        assert_eq!(parsed.to_string(), "1");
    }

    #[test]
    fn parse_expr() {
        let source = "3 * 2 + 1";
        let lexer = Lexer::new(&source);
        let parsed: Nodes = parse(&mut lexer.peekable(), 0).unwrap();
        assert_eq!(parsed.to_string(), "(+ (* 3 2) 1)");
    }
}
