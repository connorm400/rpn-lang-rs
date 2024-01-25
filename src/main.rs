use std::io::{self, Write};
use rpn_interpreter_lang::{LangState, LangError};

fn main() {
    let mut state = LangState::new(20);
    //state.add_word("squared".into(), "dup *".into());
    state.add_word("inc".into(), "1 +".into());

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
            Err(LangError::BadWordDefinitionForm) => {
                println!("Bad word definition form");
            }
            Ok(s) => {
                println!("{s}");
            },
        }
        state.print_stack();
    }
}
