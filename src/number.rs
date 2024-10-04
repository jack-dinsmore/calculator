use std::fmt::Display;

use anyhow::{anyhow, Result};
use crate::unit::Unit;

#[derive(Clone, Copy)]
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
    
    pub fn parse(s: &str) -> Self {
        unimplemented!()
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.q, self.u)
    }
}