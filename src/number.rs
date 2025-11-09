use std::fmt::Display;

use anyhow::{anyhow, Result};
use crate::{CONSTANTS, NUMBERS, UNITS, unit::Unit};

#[derive(Clone, Copy, Debug)]
pub struct Number {
    pub q: f64,
    pub u: Unit
}

impl Number {
    pub fn mul(self, b: Self) -> Result<Self> {
        Ok(Self {
            q: self.q * b.q,
            u: self.u + b.u,
        })
    }
    pub fn div(self, b: Self) -> Result<Self> {
        Ok(Self {
            q: self.q / b.q,
            u: self.u - b.u,
        })
    }
    pub fn add(self, b: Self) -> Result<Self> {
        if b.u != self.u {return Err(anyhow!("Cannot add numbers with different units"));}
        Ok(Self  {
            q: self.q + b.q,
            u: self.u
        })
    }
    pub fn sub(self, b: Self) -> Result<Self> {
        if b.u != self.u {return Err(anyhow!("Cannot subtract numbers with different units"));}
        Ok(Self {
            q: self.q - b.q,
            u: self.u
        })
    }
    pub fn neg(self) -> Result<Self> {
        Ok(Self {
            q: -self.q,
            u: self.u
        })
    }
    pub fn expon(self, b: Self) -> Result<Self> {
        if !b.u.is_one() {return Err(anyhow!("Exponents must be unitless"));}
        Ok(Self {
            q: self.q.powf(b.q),
            u: self.u * b.q
        })
    }
    
    pub fn parse(s: &str) -> Result<Self, ()> {
        if s.len() == 0 {
            return Err(());
        }

        // Find the first index of a non-number character assuming e is an exponential
        let mut first_str_index = None;
        for (i, c) in s.chars().enumerate() {
            if '0' <= c && c < '9' {continue;}
            if i > 0 && (c == '.' || c == '_' || c == '-') {continue;}
            if i > 0 && (c == 'e' || c == 'E') {
                // Check if next character is alphanumeric
                match s.chars().nth(i+1) {
                    Some(cc) => {
                        if '0' <= cc && cc < '9' {continue;} // The e is an exponential
                        if i > 0 && (cc == '.' || cc == '_' || cc == '-') {continue;} // The e is an exponential
                        () // The e is a character
                    },
                    None => (), // the e is a character
                }
            }
            first_str_index = Some(i);
            break;
        }

        match first_str_index {
            None => {
                // It's all numbers
                let q = match s.parse::<f64>() {
                    Ok(q) => q,
                    Err(_) => return Err(())
                };
                Ok(Self {
                    q,
                    u: Unit::one(),
                })
            },
            Some(index) => {
                if index == 0 {
                    // It's all letters
                    let mut parse_result =  Self::parse_unit(s);
                    if parse_result.is_err() {
                        parse_result = Self::parse_constant(s);
                    }
                    parse_result
                } else {
                    // It's half numbers, half letters
                    let q = match s[0..index].parse::<f64>() {
                        Ok(q) => q,
                        Err(_) => return Err(())
                    };
                    let q_num = Self {
                        q,
                        u: Unit::one(),
                    };
        
                    let u_num = Self::parse_unit(&s[index..])?;
        
                    match q_num.mul(u_num) {
                        Ok(n) => Ok(n),
                        Err(_) => Err(())
                    }
                }
            }
        }
    }

    fn parse_unit(s: &str) -> Result<Self, ()>{
        match UNITS.get(s) {
            Some(v) => Ok(*v),
            None => Err(())
        }
    }

    fn parse_constant(s: &str) -> Result<Self, ()>{
        if let Some(v) = NUMBERS.get(s) {
            return Ok(Self { q: *v, u: Unit::new([0., 0., 0.])})
        }
        if let Some(v) = CONSTANTS.get(s) {
            return Ok(*v)
        }
        Err(())
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.q, self.u)
    }
}