use std::{f64::consts::{E, PI}, fmt::Display};

const EULER_MASCHERONI: f64 = 0.57721566490153286060651209008240243;
const NEWTON_G: f64 = 6.6743e-8;
const HBAR: f64 = 1.05457266e-27;
const ELECTRON_MASS: f64 = 9.1093897e-28;
const PROTON_MASS: f64 = 1.6726231e-24;
const SPEED_OF_LIGHT: f64 = 2.99792458e10;
const ELECTRON_CHARGE: f64 = 4.8032068e-10;

use anyhow::{anyhow, Result};
use crate::unit::Unit;

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
        let mut first_str_index = 0;
        for (i, c) in s.chars().enumerate() {
            if !(('0' <= c && c < '9') || c == '.' || c == '_') {
                first_str_index = i;
                break;
            }
        }
        if first_str_index == 0 {
            // It's all letters
            Self::parse_unit(s)
        } else if first_str_index == s.len() {
            // It's all numbers
            let q = match s.parse::<f64>() {
                Ok(q) => q,
                Err(_) => return Err(())
            };
            Ok(Self {
                q,
                u: Unit::one(),
            })
        } else {
            // It's half numbers, half letters
            let q = match s[0..first_str_index].parse::<f64>() {
                Ok(q) => q,
                Err(_) => return Err(())
            };
            let q_num = Self {
                q,
                u: Unit::one(),
            };

            let u_num = Self::parse_unit(&s[first_str_index..])?;

            match q_num.mul(u_num) {
                Ok(n) => Ok(n),
                Err(_) => Err(())
            }
        }
    }

    fn parse_unit(s: &str) -> Result<Self, ()>{
        Ok(match s {
            // Numerical constants
            "pi" => Self { q:PI, u: Unit::new([0., 0., 0.])},
            "e" => Self { q:E, u: Unit::new([0., 0., 0.])},
            "emgamma" => Self { q:EULER_MASCHERONI, u: Unit::new([0., 0., 0.])},

            // Fundamental units
            "cm" => Self { q:1., u: Unit::new([1., 0., 0.])},
            "g" => Self { q:1., u: Unit::new([0., 1., 0.])},
            "s" => Self { q:1., u: Unit::new([0., 0., 1.])},

            // Fundamental constants
            "electron_mass" => Self { q:ELECTRON_MASS, u: Unit::new([0., 1., 0.])},
            "proton_mass" => Self { q:PROTON_MASS, u: Unit::new([0., 1., 0.])},
            "electron_charge" => Self { q:ELECTRON_CHARGE, u: Unit::new([1.5, -0.5, -1.])},
            "G" => Self { q:NEWTON_G, u: Unit::new([3., -1., -2.])},
            "hbar" => Self { q:HBAR, u: Unit::new([2., 1., -1.])},
            "c" => Self { q:SPEED_OF_LIGHT, u: Unit::new([1., 0., -1.])},

            // Other units

            _ => return Err(())
        })
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.q, self.u)
    }
}