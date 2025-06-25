use std::iter::Peekable;

#[derive(PartialEq, Debug)]
pub enum TokenKind {
    Plus,
    Minus,
    Div,
    Mult,
    Mod,
    Equal,
    Lparen,
    Rparen,
    Num(String),
    Ident(String),
    Var,
    Sin,
    Cos,
    Tan,
    Log,
    Pow,
    Illegal,
}

pub struct Token {
    pub kind: TokenKind,
    pub offset: u8, //NOTE: REMEMBER TO SUBTRACT 1 when using the offset
}

pub struct Lexer<'a> {
    chars: Peekable<std::str::Chars<'a>>,
    pub source: &'a str,
    offset: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().peekable(),
            offset: 0,
        }
    }

    fn advance(&mut self) -> Option<char> {
        while let Some(&ch) = self.chars.peek() {
            if ch.is_whitespace() {
                self.chars.next();
                self.offset += 1;
            } else {
                break;
            }
        }
        self.offset += 1;
        self.chars.next()
    }

    fn make_token(&self, kind: TokenKind) -> Token {
        Token {
            kind,
            offset: self.offset,
        }
    }

    fn match_ident(&self, ident: &str) -> TokenKind {
        match ident {
            "sin" => TokenKind::Sin,
            "cos" => TokenKind::Cos,
            "tan" => TokenKind::Tan,
            "log" => TokenKind::Log,
            "pow" => TokenKind::Pow,
            _ => TokenKind::Ident(ident.to_string()),
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut token_str = String::new();
        let c = self.advance()?;
        token_str.push(c);
        match c {
            '(' => Some(self.make_token(TokenKind::Lparen)),
            ')' => Some(self.make_token(TokenKind::Rparen)),
            '+' => Some(self.make_token(TokenKind::Plus)),
            '-' => Some(self.make_token(TokenKind::Minus)),
            '*' => Some(self.make_token(TokenKind::Mult)),
            '/' => Some(self.make_token(TokenKind::Div)),
            '%' => Some(self.make_token(TokenKind::Mod)),
            '$' => Some(self.make_token(TokenKind::Var)),
            '=' => Some(self.make_token(TokenKind::Equal)),
            'a'..='z' | 'A'..='Z' | '_' => {
                while let Some(&ch) = self.chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        token_str.push(self.advance()?)
                    } else {
                        break;
                    }
                }
                Some(self.make_token(self.match_ident(&token_str)))
            }
            '0'..='9' => {
                while let Some(&ch) = self.chars.peek() {
                    if ch.is_ascii_digit() {
                        token_str.push(self.advance()?)
                    } else if ch == '.' {
                        let dot = self.advance()?;
                        token_str.push(dot);
                        while let Some(&c) = self.chars.peek() {
                            if c.is_ascii_digit() {
                                token_str.push(self.advance()?);
                            } else {
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                }
                Some(self.make_token(TokenKind::Num(token_str)))
            }
            _ => Some(self.make_token(TokenKind::Illegal)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_parens() {
        let source = "()";
        let mut lexer = Lexer::new(source);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Lparen);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Rparen);
    }

    #[test]
    fn lex_nums() {
        let source = "1 2 3 4.5 6.99";
        let mut lexer = Lexer::new(source);
        match_number(&mut lexer, "1".to_string());
        match_number(&mut lexer, "2".to_string());
        match_number(&mut lexer, "3".to_string());
        match_number(&mut lexer, "4.5".to_string());
        match_number(&mut lexer, "6.99".to_string());
    }

    fn match_number(lexer: &mut Lexer, num: String) {
        let token_kind = lexer.next().unwrap().kind;
        match token_kind {
            TokenKind::Num(x) => assert_eq!(x, num),
            _ => panic!("Expected a Number found {:?}", token_kind),
        }
    }

    #[test]
    fn lex_arithmetic_ops() {
        let source = "+ - * / %";
        let mut lexer = Lexer::new(source);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Plus);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Minus);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Mult);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Div);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Mod);
    }

    #[test]
    fn lex_idents_and_var() {
        let source = "pow sin cos tan $hello = ";
        let mut lexer = Lexer::new(source);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Pow);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Sin);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Cos);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Tan);
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Var);
        assert_eq!(
            lexer.next().unwrap().kind,
            TokenKind::Ident("hello".to_string())
        );
        assert_eq!(lexer.next().unwrap().kind, TokenKind::Equal);
    }

    #[test]
    fn test_offset() {
        let source = "+ - * / %";
        let mut lexer = Lexer::new(source);
        verify_offset(&mut lexer, '+');
        verify_offset(&mut lexer, '-');
        verify_offset(&mut lexer, '*');
        verify_offset(&mut lexer, '/');
        verify_offset(&mut lexer, '%');
    }

    fn verify_offset(lexer: &mut Lexer, expected: char) {
        let tok = lexer.next().unwrap();
        let mut char_indices = lexer.source.char_indices();
        let (_, ch) = char_indices.nth((tok.offset - 1) as usize).unwrap();
        if ch != expected {
            panic!("Expected: {expected} got {ch} at offset {}", tok.offset);
        }
    }
}
