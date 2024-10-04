use std::io::Write;
use anyhow::{anyhow, Result};

mod util;
mod unit;
mod number;
use unit::Unit;
use number::Number;

enum Operation {
    Working,
    Number(Number),
    Identity(Box<Operation>),
    Mul(Box<Operation>, Box<Operation>),
    Div(Box<Operation>, Box<Operation>),
    Add(Box<Operation>,Box<Operation>),
    Sub(Box<Operation>,Box<Operation>),
    Neg(Box<Operation>),
    Expon(Box<Operation>,Box<Operation>),
    Func(String, Vec<Operation>)
}

impl Operation {
    pub fn relax(&self) -> Result<Number> {
        match self {
            Operation::Working => return Err(anyhow!("The string did not parse correctly")),
            Operation::Number(q) => Ok(*q),
            Operation::Identity(q) => Ok(q.relax()?),
            Operation::Mul(q1, q2) => q1.relax()?.mul(q2.relax()?),
            Operation::Div(q1, q2) => q1.relax()?.div(q2.relax()?),
            Operation::Add(q1, q2) => q1.relax()?.add(q2.relax()?),
            Operation::Sub(q1, q2) => q1.relax()?.sub(q2.relax()?),
            Operation::Neg(q) => q.relax()?.neg(),
            Operation::Expon(q1, q2) => q1.relax()?.expon(q2.relax()?),
            Operation::Func(f, v) => {
                let mut relaxed = Vec::with_capacity(v.len());
                for item in v {
                    if let Operation::Working = item {continue;}
                    let relaxed_item = item.relax()?;
                    if !relaxed_item.u.is_one() {
                        return Err(anyhow!("You cannot take a function of a variable with units"))
                    }
                    relaxed.push(relaxed_item);
                }
                match f.as_str() {
                    "sin" => {
                        if v.len() != 1 { return Err(anyhow!("sin can only take one argument")); }
                        Ok(Number { q: relaxed[0].q.sin(), u: Unit::one() })
                    },
                    "cos" => {
                        if v.len() != 1 { return Err(anyhow!("cos can only take one argument")); }
                        Ok(Number { q: relaxed[0].q.cos(), u: Unit::one() })
                    },
                    "tan" => {
                        if v.len() != 1 { return Err(anyhow!("tan can only take one argument")); }
                        Ok(Number { q: relaxed[0].q.tan(), u: Unit::one() })
                    },
                    _ => Err(anyhow!("A"))
                }
            }
        }
    }
}

fn parse(text: &str) -> Result<Operation> {
    let mut output = Operation::Identity(Box::new(Operation::Working));
    let mut current_string = "".to_owned();
    let mut last_number = None;
    for c in text.chars() {
        if ('a' <= c && c <= 'z') || c == '_' || ('1' <= c && c <= '0') {
            current_string.push(c);
        } else {
            if !current_string.is_empty() {
                // Commit the string, unless it's a function
                last_number = Some(Number::parse(&current_string));
                current_string = "".to_owned();
            }

            // Process the operation
            let replacement = if c == '*' {
                match last_number.take() {
                    Some(t) => Operation::Mul(Box::new(Operation::Number(t)), Box::new(Operation::Working)),
                    None => return Err(anyhow!("There was nothing before the '{}' symbol", c)),
                }
            } else if c == '/' {
                match last_number.take() {
                    Some(t) => Operation::Div(Box::new(Operation::Number(t)), Box::new(Operation::Working)),
                    None => return Err(anyhow!("There was nothing before the '{}' symbol", c)),
                }
            } else if c == '+' {
                match last_number.take() {
                    Some(t) => Operation::Add(Box::new(Operation::Number(t)), Box::new(Operation::Working)),
                    None => return Err(anyhow!("There was nothing before the '{}' symbol", c)),
                }
            } else if c == '-' {
                match last_number.take() {
                    Some(t) => Operation::Sub(Box::new(Operation::Number(t)), Box::new(Operation::Working)),
                    None => Operation::Neg(Box::new(Operation::Working)),
                }
            } else if c == '^' {
                match last_number.take() {
                    Some(t) => Operation::Expon(Box::new(Operation::Number(t)), Box::new(Operation::Working)),
                    None => return Err(anyhow!("There was nothing before the '{}' symbol", c)),
                }
            } else if c == '+' {
                match last_number.take() {
                    Some(t) => Operation::Add(Box::new(Operation::Number(t)), Box::new(Operation::Working)),
                    None => return Err(anyhow!("There was nothing before the '{}' symbol", c)),
                }
            } else if c == '\n' {
                match last_number.take() {
                    Some(t) => Operation::Number(t),
                    None => break,
                }
            } else if c == ')' {
                // TODO custom code to remove the last working operation inside a function if it exists
                continue
            } else if c == ']' {
                continue;
            } else if c == '(' {
                match last_number.take() {
                    Some(t) => Operation::Mul(Box::new(Operation::Number(t)), Box::new(Operation::Working)),
                    None => {
                        if current_string.is_empty() {
                            // Parenthesis
                            Operation::Identity(Box::new(Operation::Working))
                        } else {
                            // Start function
                            let func = current_string.clone();
                            current_string = "".to_owned();
                            Operation::Func(func, vec![Operation::Working])
                        }
                    },
                }
            } else if c == ',' {
                // TODO custom code to replace the curret working operation in a function and add a new one after it
                continue;
            } else {
                return Err(anyhow!("Unrecognized character {}", c))
            };

            // TODO replace Working with replacement
        }
    }
    Ok(output)
}

fn main() {
    loop {
        print!(">>> ");
        let _ = std::io::stdout().flush();
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input).expect("Did not enter a valid string");

        if input == "exit\n" {
            break;
        }
        if input == "\n" {
            continue;
        }
    
        let parsed = match parse(&input) {
            Ok(o) => o,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        let number = match parsed.relax() {
            Ok(n) => n,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        println!("{}", number);
    }
}
