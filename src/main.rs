use std::io::{self, Write};
use std::vec::Drain;

pub mod tok;

#[derive(Debug, Eq, PartialEq)]
enum Value {
    Nil,
    Number(u64),
    Atom(Box<str>),
    String(Box<str>),
    List(Vec<Value>)
}

fn to_ast<'a>(tokens: &mut Drain<'a, tok::Token>) -> Value {
    let mut ast = Vec::new();
    let mut proceeding = true;
    while proceeding {
        match tokens.next() {
            Some(token) => {
                match token {
                    tok::Token::ParenR => {
                        proceeding = false;
                    }
                    tok::Token::Identifier(s) => {
                        ast.push(Value::Atom(s));
                    }
                    tok::Token::String(s) => {
                        ast.push(Value::String(s));
                    }
                    tok::Token::ParenL => {
                        ast.push(to_ast(tokens));
                    }
                }
            }
            None => {
                proceeding = false;
            }
        }
    }
    Value::List(ast)
}

fn eval(ast: Value) -> Value {
    ast
}

#[cfg(test)]
mod tests {
  use super::{Value, eval};
#[test]
    fn test_eval() {
        assert!(eval(Value::Number(1)) != eval(Value::Number(2)));
    }
}

fn print_prompt(prompt_string: &str) {
    print!("{}> ", prompt_string);
    io::stdout().flush().unwrap(); // ensure prompt is printed before asking for input
}

fn main() {
    let prompt_text = "repl";
    let debug_display_tokens = true;

    loop {
        print_prompt(prompt_text);

        let mut input_line = String::new();

        io::stdin().read_line(&mut input_line)
            .expect("Failed to read line");

        let mut tokens = tok::tokenize(&input_line);
        if debug_display_tokens {
            println!("tokens:");
            println!("{:?}", tokens);
        }
        let ast = to_ast(&mut tokens.drain(..));
        let res = eval(ast);

        println!("ast:");
        println!("{:?}", res);
    }
}
