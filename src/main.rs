use std::io::{self, Write};

fn eval(command: String) -> String {
    command
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
        
        let res = eval(command);

        println!("{}", res);
    }
}
