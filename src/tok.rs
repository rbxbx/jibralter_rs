use std::str;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
  ParenL, ParenR,
  Identifier(Box<str>),
  String(Box<str>)
}

pub fn tokenize(s: &str) -> Vec<Token> {
  let mut tokens = Vec::new();
  let mut chars = s.chars();
  while let Some(c) = chars.next() {
    tokens.push(match c {
      '(' => Token::ParenL,
      ')' => Token::ParenR,
      '"' => {
        let s: String = chars.take_while(|c| c != '"').collect();
        Token::String(s.into_boxed_str())
      }
      c if c.is_alphabetic() => {
        let id: String = chars.take_while(|c| !c.is_whitespace()).collect();
        Token::Identifier(id.into_boxed_str())
      }
      c if c.is_whitespace() => continue,
      c => panic!("unexpected character {:?}", c)
    });
  }
  tokens
}
