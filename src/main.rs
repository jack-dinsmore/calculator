use std::{io::Write, ops::DerefMut};
use anyhow::{anyhow, Result};

mod util;
mod unit;
mod number;
use number::Number;
use unit::Unit;

const MAX_ARGS: usize = 8;

#[derive(Clone, Debug)]
enum Operator {
    Working,
    Number(Number),
    Identity,
    Mul,
    Div,
    Add,
    Sub,
    Neg,
    Expon,
    Func(String)
}

impl Operator {
    fn get_label(&self) -> usize {
        match self {
            Operator::Working => 0,
            Operator::Number(_) => 0,
            Operator::Identity => 5,
            Operator::Mul => 2,
            Operator::Div => 2,
            Operator::Add => 1,
            Operator::Sub => 1,
            Operator::Neg => 4,
            Operator::Expon => 3,
            Operator::Func(_) => 0,
        }
    }
}

impl PartialEq for Operator {
    fn eq(&self, other: &Self) -> bool {
        self.get_label() == other.get_label()
    }
}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.get_label().cmp(&other.get_label()))
    }
}

#[derive(Clone, Debug)]
struct Instruction {
    operator: Operator,
    parent: *mut Instruction,
    children: Option<Box<[Instruction; MAX_ARGS]>>,
}

impl Instruction {
    fn head() -> Self {
        Self {
            operator: Operator::Identity,
            parent: std::ptr::null::<Instruction>() as *mut Instruction,
            children: Self::empty_children(std::ptr::null::<Instruction>() as *mut Instruction),
        }
    }

    fn empty_children(parent: *mut Instruction) -> Option<Box<[Instruction; MAX_ARGS]>> {
        let child = Instruction {
            operator: Operator::Working,
            parent,
            children: None,
        };
        Some(Box::new([child.clone(), child.clone(), child.clone(), child.clone(), child.clone(), child.clone(), child.clone(), child.clone()]))
    }

    fn calculate(&self) -> Result<Number> {
        match &self.operator {
            Operator::Working => Err(anyhow!("Could not parse string")),
            Operator::Number(number) => {
                Ok(*number)
            },
            Operator::Identity => {
                let children = self.children.as_ref().unwrap();
                Ok(children[0].calculate()?)
            },
            Operator::Mul => {
                let children = self.children.as_ref().unwrap();
                children[0].calculate()?.mul(children[1].calculate()?)
            },
            Operator::Div => {
                let children = self.children.as_ref().unwrap();
                children[0].calculate()?.div(children[1].calculate()?)
            },
            Operator::Add => {
                let children = self.children.as_ref().unwrap();
                children[0].calculate()?.add(children[1].calculate()?)
            },
            Operator::Sub => {
                let children = self.children.as_ref().unwrap();
                children[0].calculate()?.sub(children[1].calculate()?)
            },
            Operator::Neg => {
                let children = self.children.as_ref().unwrap();
                children[0].calculate()?.neg()
            },
            Operator::Expon => {
                let children = self.children.as_ref().unwrap();
                children[0].calculate()?.expon(children[1].calculate()?)
            },
            Operator::Func(func) => {
                let children = self.children.as_ref().unwrap();
                Ok(match func.as_str() {
                    "sqrt" => {
                        let n = children[0].calculate()?;
                        Number { q: n.q.sqrt(), u: Unit::one() * 0.5 }
                    },
                    "cbrt" => {
                        let n = children[0].calculate()?;
                        Number { q: n.q.sqrt(), u: Unit::one() * (1. / 3.) }
                    },
                    "sin" => {
                        let n = children[0].calculate()?;
                        if !n.u.is_one() {return Err(anyhow!("Functions should take unitless numbers"));}
                        Number { q: n.q.sin(), u: Unit::one() }
                    },
                    "cos" => {
                        let n = children[0].calculate()?;
                        if !n.u.is_one() {return Err(anyhow!("Functions should take unitless numbers"));}
                        Number { q: n.q.cos(), u: Unit::one() }
                    },
                    "tan" => {
                        let n = children[0].calculate()?;
                        if !n.u.is_one() {return Err(anyhow!("Functions should take unitless numbers"));}
                        Number { q: n.q.tan(), u: Unit::one() }
                    },
                    "asin" | "arcsin" => {
                        let n = children[0].calculate()?;
                        if !n.u.is_one() {return Err(anyhow!("Functions should take unitless numbers"));}
                        Number { q: n.q.asin(), u: Unit::one() }
                    },
                    "acos" | "arccos" => {
                        let n = children[0].calculate()?;
                        if !n.u.is_one() {return Err(anyhow!("Functions should take unitless numbers"));}
                        Number { q: n.q.acos(), u: Unit::one() }
                    },
                    "atan" | "arctan" => {
                        let n = children[0].calculate()?;
                        if !n.u.is_one() {return Err(anyhow!("Functions should take unitless numbers"));}
                        Number { q: n.q.atan(), u: Unit::one() }
                    },
                    _ => return Err(anyhow!("The function {} is not supported", func))
                })
            },
        }
    }
    
    /// Inserts a new instruction at the closest point in the tree to cursor allowed by the order of operations. The new instruction has operator op and its first child is the child it replaced.
    fn insert_in_parent(cursor: &mut Instruction, op: Operator) -> &mut Instruction {
        // Navigate to the closest parent of cursor that contains an operation lower in order or equal to op
        let mut parent = unsafe {&mut *cursor.parent};
        let mut child = cursor as *mut Instruction;
        while parent.operator <= op {
            child = parent;
            parent = unsafe {&mut *cursor.parent};
        }
        let parent_ptr = parent as *mut Instruction;

        // Get the index of the child to be replaced
        let children = parent.children.as_mut().unwrap();
        let mut child_index = 0;
        while child_index < 8 {
            if std::ptr::eq(&children[child_index] as *const Instruction, child) { break; }
            child_index += 1;
        }

        // Move the child into a list of children for the new struct
        let mut new_children = Instruction::empty_children(&mut children[child_index]);
        (*new_children.as_mut().unwrap())[0] = (*children)[child_index].clone();

        // Replace the original child
        children[child_index] = Instruction {
            operator: op,
            parent: parent_ptr,
            children: new_children,
        };
        let new_parent_ptr = &mut children[child_index] as *mut Instruction;

        // Relabel the child's parent pointer
        children[child_index].children.as_mut().unwrap()[0].parent = new_parent_ptr;

        &mut children[child_index]
    }

    /// Return the next child in the list
    fn get_next_child(&self) -> &mut Instruction {
        let parent = unsafe {&mut *self.parent};
        let children = parent.children.as_mut().unwrap();

        let mut child_index = 0;
        while child_index < 8 {
            if std::ptr::eq(self as *const Instruction, self) { break; }
            child_index += 1;
        }
        child_index += 1;
        &mut children[child_index]
    }

    /// Return the next working child in the of this instruction
    fn get_first_working_child(&mut self) -> &mut Instruction {
        let children = self.children.as_mut().unwrap();
        let mut child_index = 0;
        loop {
            if let Operator::Working = children[child_index].operator { break; }
            child_index += 1;
        }
        &mut children[child_index]
    }

    fn get_parent(&self) -> &mut Instruction {
        unsafe {&mut *self.parent}
    }
}

fn parse(text: &str) -> Result<Instruction> {
    let mut output = Instruction::head();
    let output_ptr = &mut output as *mut Instruction;
    let children = output.children.as_mut().unwrap().deref_mut();
    for child in children {
        child.parent = output_ptr;
    }
    let mut cursor = &mut output.children.as_mut().unwrap().deref_mut()[0];

    let mut parentheses_cursors = Vec::new();

    let mut current_string = "".to_owned();
    for c in text.chars() {
        dbg!(c);
        if ('a' <= c && c <= 'z') || 'A' <= c && c <= 'Z' || c == '_' || c == '.' || ('0' <= c && c <= '9') {
            current_string.push(c);
        } else {
            if !current_string.is_empty() {
                // Commit the string
                match Number::parse(&current_string) {
                    Ok(number) => cursor.operator = Operator::Number(number),
                    Err(()) => cursor.operator = Operator::Func(current_string),
                }
                current_string = "".to_owned();
            }

            if c == ' ' {continue;}

            // Process the operation
            if c == '*' {
                let child = Instruction::insert_in_parent(cursor, Operator::Mul);
                cursor = child.get_first_working_child();
            } else if c == '/' {
                let child = Instruction::insert_in_parent(cursor, Operator::Div);
                cursor = child.get_first_working_child();
            } else if c == '+' {
                let child = Instruction::insert_in_parent(cursor, Operator::Add);
                cursor = child.get_first_working_child();
            } else if c == '-' {
                // TODO handle negation
                let child = Instruction::insert_in_parent(cursor, Operator::Sub);
                cursor = child.get_first_working_child();
            } else if c == '^' {
                let child = Instruction::insert_in_parent(cursor, Operator::Expon);
                cursor = child.get_first_working_child();
            } else if c == '\n' {
                break;
            } else if c == ')' {
                if let Operator::Func(_) = cursor.get_parent().operator {
                    cursor = cursor.get_parent();
                } else {
                    cursor = match parentheses_cursors.pop() {
                        Some(c) => c,
                        None => return Err(anyhow!("The ) was not opened")),
                    };
                }
            } else if c == ']' {
                cursor = match parentheses_cursors.pop() {
                    Some(c) => c,
                    None => return Err(anyhow!("The ] was not opened")),
                };
            } else if c == '(' {
                if let Operator::Func(_) = cursor.operator {
                    // Opening a funcion
                } else {
                    let child = Instruction::insert_in_parent(cursor, Operator::Identity);
                    cursor = child
                }
            } else if c == ',' {
                if let Operator::Func(_) = cursor.get_parent().operator {} else {
                    return Err(anyhow!("You cannot use , except in a function"))
                }
                cursor = cursor.get_next_child();
            } else {
                return Err(anyhow!("Unrecognized character {}", c))
            }
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

        let number = match parsed.calculate() {
            Ok(n) => n,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        println!("{}", number);
    }
}
