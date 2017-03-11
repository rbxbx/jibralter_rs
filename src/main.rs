use std::io::{self, Write};

#[derive(Debug, Eq, PartialEq)]
enum Value { Nil, Number(u64), Atom(Box<str>), String(Box<str>) }

fn eval(command: Value) -> Value {
    command
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

    loop {
        print_prompt(prompt_text);

        let mut command = String::new();

        io::stdin().read_line(&mut command)
            .expect("Failed to read line");

        let res = eval(Value::String(command.into_boxed_str()));

        println!("{:?}", res);
    }
}
