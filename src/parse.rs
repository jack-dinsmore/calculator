use anyhow::{Result, anyhow};
use crate::number::Number;
use crate::instruction::{Operator, Instruction};


pub fn parse(text: &str) -> Result<Instruction> {
    let text = if text.ends_with("\n") {
        text.to_owned()
    } else {
        format!("{}\n", text)
    };
    let mut output = Instruction::head();
    let mut cursor = output.get_first_working_child();

    let mut current_string = "".to_owned();
    for c in text.chars() {
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
                let child = if let Operator::Working = cursor.operator {
                    Instruction::insert_in_parent(cursor, Operator::Neg)
                } else {
                    Instruction::insert_in_parent(cursor, Operator::Sub)
                };
                cursor = child.get_first_working_child();
            } else if c == '^' {
                let child = Instruction::insert_in_parent(cursor, Operator::Expon);
                cursor = child.get_first_working_child();
            } else if c == '\n' {
                break;
            } else if c == ')' {
                cursor = cursor.close_parentheses()?;
            } else if c == '(' {
                if let Operator::Func(_) = cursor.operator {
                    cursor = cursor.get_first_working_child();
                }
                let child = Instruction::insert_in_parent(cursor, Operator::Parentheses);
                cursor = child.get_first_working_child();
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
