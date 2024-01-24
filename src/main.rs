use std::io::{self, Write};
use rpn_interpreter_lang::{LangState, LangError};

fn main() {
    let mut state = LangState::new(20);
    state.add_name("squared", "dup *");
    state.add_name("inc", "1 +");

    println!("version 0.0.0.0.1\ntype 'bye' to exit");
    loop {
        let mut input = String::new();
        print!(">>> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .unwrap();

        match state.solve(input.trim()) {
            Err(LangError::NameNotFound) => {
                println!("Name not Found");
            },
            Err(LangError::StackUnderflow) => {
                println!("Stack underflow!");
            },
            Err(LangError::BadNumberParse) => {
                println!("Number couldn't be parsed");
            }
            Ok(s) => {
                println!("{s}");
            },
        }
        state.print_stack();
    }
}
