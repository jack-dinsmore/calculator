use anyhow::{Result, bail, anyhow};
use crate::{FUNCTIONS, number::Number};

const MAX_ARGS: usize = 2;

#[derive(Clone, Debug)]
pub enum Operator {
    Head,
    Working,
    Number(Number),
    Func(String),
    Parentheses,
    Mul,
    Div,
    Add,
    Sub,
    Neg,
    Expon,
}

impl Operator {
    /// Used to determine the order of operations
    pub fn get_label(&self) -> usize {
        match self {
            Operator::Head => usize::MAX,
            Operator::Func(_) => 5,
            Operator::Parentheses => 5,
            Operator::Neg => 4,
            Operator::Expon => 3,
            Operator::Mul => 2,
            Operator::Div => 2,
            Operator::Add => 1,
            Operator::Sub => 1,
            Operator::Working => 0,
            Operator::Number(_) => 0,
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
pub struct Instruction {
    pub operator: Operator,
    pub parent: *mut Instruction,
    pub children: Option<Box<[Instruction; MAX_ARGS]>>,
}

impl Instruction {
    pub fn head() -> Self {
        Self {
            operator: Operator::Head,
            children: None,
            parent: std::ptr::null::<Instruction>() as *mut Instruction,
        }
    }

    fn empty_children(parent: *mut Instruction) -> Option<Box<[Instruction; MAX_ARGS]>> {
        let child = Instruction {
            operator: Operator::Working,
            parent,
            children: None,
        };
        Some(Box::new([child.clone(), child.clone()]))
    }

    pub fn calculate(&self) -> Result<Number> {
        match &self.operator {
            Operator::Working => Err(anyhow!("Could not parse string")),
            Operator::Number(number) => {
                Ok(*number)
            },
            Operator::Parentheses | Operator::Head => {
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
                let children = match self.children.as_ref() {
                    Some(c) => c,
                    None => bail!("The name {} is neither a function nor a variable", func)
                };
                Ok(match FUNCTIONS.get(func.as_str()) {
                    Some((f, unit_mult)) => {
                        let n = children[0].calculate()?;
                        Number { q: f(n.q), u: n.u * *unit_mult }
                    },
                    None => return Err(anyhow!("The function {} is not supported", func))
                })
            },
        }
    }
    
    /// Inserts a new instruction at the closest parent allowed by the order of operations. The new instruction has operator op and its first child is the child it replaced.
    pub fn insert_in_parent(cursor: &mut Instruction, op: Operator) -> &mut Instruction {
        // Navigate to the closest parent of cursor that contains an operation lower in order or equal to op
        let mut parent = unsafe {&mut *cursor.parent};
        let mut child = cursor as *mut Instruction;
        while parent.operator < op {
            child = parent;
            parent = unsafe {&mut *parent.parent};
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
    pub fn get_next_child(&self) -> &mut Instruction {
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
    pub fn get_first_working_child(&mut self) -> &mut Instruction {
        if self.children.is_none() {
            self.children = Self::empty_children(self);
        }
        let children = self.children.as_mut().unwrap();
        let mut child_index = 0;
        loop {
            if let Operator::Working = children[child_index].operator { break; }
            child_index += 1;
        }
        &mut children[child_index]
    }

    pub fn get_parent(&self) -> &mut Instruction {
        unsafe {&mut *self.parent}
    }
    
    pub fn close_parentheses(&mut self) -> Result<&mut Instruction> {
        let mut child = self;
        while child.operator < Operator::Parentheses {
            child = unsafe{&mut *child.parent};
        }
        if let Operator::Parentheses = child.operator {
            Ok(child)
        } else {
            Err(anyhow!("Too many )"))
        }
    }
}