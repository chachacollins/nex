use crate::lexer::{Lexer, Token, TokenKind};
use miette::{Diagnostic, Result, SourceSpan};
use std::fmt;
use std::iter::Peekable;
use std::str::FromStr;
use thiserror::Error;

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
    Number(u8, f64),
    Negative(Box<Nodes>),
    Positive(Box<Nodes>),
    Operator(OperatorNode<Nodes>),
}

impl Node for Nodes {}
impl fmt::Display for Nodes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Nodes::Number(_, num) => write!(f, "{}", num),
            Nodes::Negative(node) => write!(f, "-{}", node),
            Nodes::Positive(node) => write!(f, "+{}", node),
            Nodes::Operator(op) => op.fmt(f),
        }
    }
}

fn get_precedence(kind: &TokenKind) -> (u8, u8) {
    use TokenKind::*;
    match kind {
        Plus => (3, 1),
        Mod => (0, 1),
        Minus => (3, 1),
        Mult | Div => (0, 2),
        _ => unreachable!(),
    }
}

#[derive(Error, Debug, Diagnostic)]
#[error("Unclosed brackets!")]
#[diagnostic(help("try closing brackets next time?"))]
struct UnclosedBracket {
    #[source_code]
    src: String,
    #[label("This bit here")]
    bad_bit: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Failed to parse number!")]
#[diagnostic(help("try entering a valid number"))]
struct NumParseError {
    #[source_code]
    src: String,
    #[label("This right here")]
    bad_bit: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Unexpected Token!")]
#[diagnostic(help("Enter help command for a list of valid operations"))]
struct UnexpectedToken {
    #[source_code]
    src: String,
    #[label("This token here")]
    bad_bit: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Error: Unexpected Eof!")]
#[diagnostic(help("try writing complete expression(type help for more info)"))]
struct UnexpectedEof {}

pub fn parse(src: &str, lexer: &mut Peekable<Lexer>, prev_precedence: u8) -> Result<Nodes> {
    use TokenKind::*;
    let token = lexer.next().ok_or(UnexpectedEof {})?;
    let mut lhs = match token.kind {
        Num(num) => Nodes::Number(
            token.offset,
            f64::from_str(&num).map_err(|_| NumParseError {
                src: src.to_string(),
                bad_bit: ((token.offset - num.len() as u8) as usize, num.len()).into(),
            })?,
        ),
        Lparen => {
            let expression = parse(src, lexer, 0)?;
            let consumed = lexer.next().ok_or(UnclosedBracket {
                src: src.to_string(),
                bad_bit: ((token.offset - 1) as usize, 1).into(),
            })?;
            if consumed.kind != Rparen {
                Err(UnclosedBracket {
                    src: src.to_string(),
                    bad_bit: ((token.offset - 1) as usize, 1).into(),
                })?;
            }
            expression
        }
        Minus => {
            let (prefix, _) = get_precedence(&TokenKind::Minus);
            let expression = parse(src, lexer, prefix)?;
            Nodes::Negative(Box::new(expression))
        }
        Plus => {
            let (prefix, _) = get_precedence(&TokenKind::Plus);
            let expression = parse(src, lexer, prefix)?;
            Nodes::Positive(Box::new(expression))
        }
        _ => {
            return Err(UnexpectedToken {
                src: src.to_string(),
                bad_bit: ((token.offset - 1) as usize, 1).into(),
            })?;
        }
    };
    while let Some(next_token) = lexer.peek() {
        match next_token.kind {
            Plus | Minus | Div | Mod | Mult => {
                let (_, precedence) = get_precedence(&next_token.kind);
                if precedence <= prev_precedence {
                    break;
                } else {
                    let consumed_token = lexer.next().unwrap();
                    let right_node = parse(src, lexer, precedence)?;
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
        let parsed: Nodes = parse(lexer.source, &mut lexer.peekable(), 0).unwrap();
        assert_eq!(parsed.to_string(), "1");
    }

    #[test]
    fn parse_expr() {
        let source = "3 * 2 + 1";
        let lexer = Lexer::new(&source);
        let parsed: Nodes = parse(lexer.source, &mut lexer.peekable(), 0).unwrap();
        assert_eq!(parsed.to_string(), "(+ (* 3 2) 1)");
    }

    #[test]
    fn parse_negative_pref() {
        let source = "-3 + 2";
        let lexer = Lexer::new(&source);
        let parsed: Nodes = parse(lexer.source, &mut lexer.peekable(), 0).unwrap();
        assert_eq!(parsed.to_string(), "(+ -3 2)");
    }

    #[test]
    fn parse_pos_pref() {
        let source = "+(-3 + 2)";
        let lexer = Lexer::new(&source);
        let parsed: Nodes = parse(lexer.source, &mut lexer.peekable(), 0).unwrap();
        assert_eq!(parsed.to_string(), "+(+ -3 2)");
    }
}
