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
  let mut paren_depth = 0u32;
  while let Some(c) = chars.next() {
    tokens.push(match c {
      '(' => {
        paren_depth += 1;
        Token::ParenL
      }
      ')' => {
        paren_depth = paren_depth.checked_sub(1).expect("unmatched right paren");
        Token::ParenR
      }
      '"' => {
        let s: String = (&mut chars).take_while(|&c| c != '"').collect();
        Token::String(s.into_boxed_str())
      }
      c if c.is_alphabetic() => {
        let id: String = (&mut chars).take_while(|c| !c.is_whitespace()).collect();
        Token::Identifier(id.into_boxed_str())
      }
      c if c.is_whitespace() => continue,
      c => panic!("unexpected character {:?}", c)
    });
  }
  tokens
}
