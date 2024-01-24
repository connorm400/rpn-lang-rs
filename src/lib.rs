use std::error::Error;
use std::fmt::Formatter;
use std::process::exit;
use std::collections::HashMap;

pub struct LangState<'a> {
    stack: Vec<i32>,
    env: HashMap<&'a str, &'a str>
}

impl<'a> LangState<'a> {
    pub fn new(stack_capacity: usize) -> Self {
        LangState {
            stack: Vec::with_capacity(stack_capacity),
            env: HashMap::new(),
        }
    }

    pub fn solve(&mut self, input: &str) -> Result<String, LangError> {
        let mut output = String::new();
        for (i, symbol) in input.split(' ').enumerate() {
            match symbol {
                _ if symbol.parse::<i32>().is_ok() => {
                    self.stack.push(symbol.parse().map_err(|_| LangError::BadNumberParse)?);
                },
                "+" => {
                    let a = self.stack.pop().ok_or(LangError::StackUnderflow)?;
                    let b = self.stack.pop().ok_or(LangError::StackUnderflow)?;
                    self.stack.push(a + b);
                },
                "-" => {
                    let a = self.stack.pop().ok_or(LangError::StackUnderflow)?;
                    let b = self.stack.pop().ok_or(LangError::StackUnderflow)?;
                    self.stack.push(a - b);
                }
                "*" => {
                    let a = self.stack.pop().ok_or(LangError::StackUnderflow)?;
                    let b = self.stack.pop().ok_or(LangError::StackUnderflow)?;
                    self.stack.push(a * b);
                }
                "/" => {
                    let a = self.stack.pop().ok_or(LangError::StackUnderflow)?;
                    let b = self.stack.pop().ok_or(LangError::StackUnderflow)?;
                    self.stack.push(b / a);
                }
                "." => {
                    output.push_str(&format!("{} ", self.stack.pop().ok_or(LangError::StackUnderflow)?));
                },
                "dup" => {
                    let x = self.stack.pop().ok_or(LangError::StackUnderflow)?;
                    self.stack.push(x);
                    self.stack.push(x);
                }
                "bye" => {
                    println!("Exiting...");
                    exit(0);
                }
                "index" => {
                    output.push_str(&format!("{i}"));
                },
                s if self.env.contains_key(s) => {
                    output = self.solve(
                        self.env.get(s).ok_or(LangError::NameNotFound)?)?;
                },
                _ => {
                    return Err(LangError::NameNotFound);
                },
            }
        }

        Ok(output)
    }

    pub fn print_stack(&self) {
        println!("Stack: {:?}", self.stack);
    }

    pub fn add_name(&mut self, name: &'a str, contents: &'a str) {
        self.env.insert(name, contents);
    }
}

#[derive(Debug, PartialEq)]
pub enum LangError {
    StackUnderflow,
    NameNotFound,
    BadNumberParse,
}

impl std::fmt::Display for LangError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for LangError {}