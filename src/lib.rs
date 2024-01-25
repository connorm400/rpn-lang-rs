use std::error::Error;
use std::fmt::Formatter;
use std::process::exit;
use std::collections::HashMap;
use std::iter::Skip;
use std::str::Split;

pub struct LangState {
    stack: Vec<i32>,
    dictionary: HashMap<String, String>
}

impl LangState {
    pub fn new(stack_capacity: usize) -> Self {
        LangState {
            stack: Vec::with_capacity(stack_capacity),
            dictionary: HashMap::new(),
        }
    }

    pub fn solve(&mut self, input: &str) -> Result<String, LangError> {
        use LangError as E;
        let mut output = String::new();
        let mut skip = false;
        for (i, symbol) in input.split(' ').enumerate() {
            // bad code
            if skip {
                if symbol == ";" {
                    skip = false;
                }
                continue;
            }
            match symbol.to_lowercase().as_str() {
                _ if symbol.parse::<i32>().is_ok() => {
                    self.stack.push(symbol.parse().map_err(|_| E::BadNumberParse)?);
                },
                "+" => {
                    let a = self.stack.pop().ok_or(E::StackUnderflow)?;
                    let b = self.stack.pop().ok_or(E::StackUnderflow)?;
                    self.stack.push(a + b);
                },
                "-" => {
                    let a = self.stack.pop().ok_or(E::StackUnderflow)?;
                    let b = self.stack.pop().ok_or(E::StackUnderflow)?;
                    self.stack.push(a - b);
                }
                "*" => {
                    let a = self.stack.pop().ok_or(E::StackUnderflow)?;
                    let b = self.stack.pop().ok_or(E::StackUnderflow)?;
                    self.stack.push(a * b);
                }
                "/" => {
                    let a = self.stack.pop().ok_or(E::StackUnderflow)?;
                    let b = self.stack.pop().ok_or(E::StackUnderflow)?;
                    self.stack.push(b / a);
                }
                "." => {
                    output.push_str(&format!("{} ", self.stack.pop().ok_or(E::StackUnderflow)?));
                },
                ">" => {
                    let a = self.stack.pop().ok_or(E::StackUnderflow)?;
                    let b = self.stack.pop().ok_or(E::StackUnderflow)?;
                    self.stack.push((b > a) as i32);
                },
                "<" => {
                    let a = self.stack.pop().ok_or(E::StackUnderflow)?;
                    let b = self.stack.pop().ok_or(E::StackUnderflow)?;
                    self.stack.push((b < a) as i32);
                }
                "=" => {
                    let a = self.stack.pop().ok_or(E::StackUnderflow)?;
                    let b = self.stack.pop().ok_or(E::StackUnderflow)?;
                    self.stack.push((a == b) as i32);
                },
                "dup" => {
                    let x = self.stack.pop().ok_or(E::StackUnderflow)?;
                    self.stack.push(x);
                    self.stack.push(x);
                },
                "swap" => {
                    let a = self.stack.pop().ok_or(E::StackUnderflow)?;
                    let b = self.stack.pop().ok_or(E::StackUnderflow)?;
                    self.stack.push(a);
                    self.stack.push(b);
                },
                "clear" => {
                    self.stack.clear();
                }
                "bye" => {
                    println!("Exiting...");
                    exit(0);
                },
                ":" => {
                    self.user_define_word(input.split(' ').skip(i + 1))?;
                    output = "defined".into();
                    skip = true;
                },
                s if self.dictionary.contains_key(s) => {
                    let contents: String = self.dictionary.get(s).ok_or(LangError::NameNotFound)?.into();
                    output = self.solve(&contents)?;
                },
                "" => { continue; },
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

    pub fn add_word(&mut self, word: String, contents: String) {
        self.dictionary.insert(word, contents);
    }

    // bad code
    fn user_define_word(&mut self, mut coll: Skip<Split<'_, char>>) -> Result<(), LangError> {
        let name = coll.next().ok_or(LangError::BadWordDefinitionForm)?;
        let mut contents = String::new();
        while let Some(x) = coll.next() {
            if x == ";" {
                break;
            }
            contents.push_str(&format!("{} ", x));
        }

        self.add_word(name.into(), contents);
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum LangError {
    StackUnderflow,
    NameNotFound,
    BadNumberParse,
    BadWordDefinitionForm,
}

impl std::fmt::Display for LangError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for LangError {}