use std::{fmt::Display, ops::{Add, Mul, Sub}};
use crate::util::{round_eps, EPSILON};

#[derive(Clone, Copy)]
pub struct Unit {
    cm: f64,
    g: f64,
    s: f64,
}

impl Unit {
    pub fn parse(unit: &str) -> Self {
        unimplemented!()
    }

    pub fn one() -> Self {
        Self {
            cm: 0.,
            g: 0.,
            s: 0.,
        }
    }

    pub fn is_one(&self) -> bool {
        self.cm.abs() < EPSILON && self.g.abs() < EPSILON && self.s.abs() < EPSILON
    }
}

impl Add for Unit {
    type Output=Self;
    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
impl Mul<f64> for Unit {
    type Output=Self;
    fn mul(self, rhs: f64) -> Self::Output {
        todo!()
    }
}
impl Sub for Unit {
    type Output=Self;
    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        (self.cm - other.cm).abs() < EPSILON && (self.g - other.g).abs() < EPSILON && (self.s - other.s).abs() < EPSILON
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cm_power = round_eps(self.cm);
        let g_power = round_eps(self.g);
        let s_power = round_eps(self.s);
        let mut out = "".to_owned();

        if cm_power == "1" {
            out = format!("{} cm", out)
        } else if cm_power == "0" {
            out = format!("{}", out)
        } else {
            out = format!("{} cm^{}", out, cm_power)
        }

        if g_power == "1" {
            out = format!("{} g", out)
        } else if g_power == "0" {
            out = format!("{}", out)
        } else {
            out = format!("{} g^{}", out, g_power)
        }

        if s_power == "1" {
            out = format!("{} s", out)
        } else if s_power == "0" {
            out = format!("{}", out)
        } else {
            out = format!("{} s^{}", out, s_power)
        }

        write!(f, "{}", out)
    }
}