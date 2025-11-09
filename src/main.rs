use std::collections::HashMap;
use lazy_static::lazy_static;
use rustyline::{DefaultEditor, Config, EditMode};

mod util;
mod unit;
mod number;
mod instruction;
mod parse;
use parse::parse;

use crate::{number::Number, unit::Unit};

lazy_static! {
    static ref NUMBERS: HashMap<&'static str, f64> = {
        let mut a = HashMap::new();
        a.insert("pi", std::f64::consts::PI);
        a.insert("e", std::f64::consts::E);
        a.insert("egamma", 0.5772156649015328606065120900824024310421593359);
        a
    };
    static ref CONSTANTS: HashMap<&'static str, Number> = {
        let mut a = HashMap::new();
        a.insert("electron_mass",  Number { q: 9.1093897e-28, u: Unit::new([0., 1., 0.])});
        a.insert("proton_mass", Number { q: 1.6726231e-24, u: Unit::new([0., 1., 0.])});
        a.insert("electron_charge", Number { q: 4.8032068e-10, u: Unit::new([1.5, -0.5, -1.])});
        a.insert("GN", Number { q: 6.6743e-8, u: Unit::new([3., -1., -2.])});
        a.insert("h", Number { q: (2.*std::f64::consts::PI)*1.05457266e-27, u: Unit::new([2., 1., -1.])});
        a.insert("hbar", Number { q: 1.05457266e-27, u: Unit::new([2., 1., -1.])});
        a.insert("c", Number { q: 2.99792458e10, u: Unit::new([1., 0., -1.])});
        a.insert("kb", Number { q: 1.3807e-16, u: Unit::new([2., 1., -2.])}); // Also times K^-1
        a
    };
    static ref FUNCTIONS: HashMap<&'static str, (fn(f64)->f64, f64)> = {
        let mut a: HashMap<&'static str, (fn(f64)->f64, f64)> = HashMap::new();
        a.insert("sqrt", (|x| x.sqrt(), 0.5));
        a.insert("cbrt", (|x| x.cbrt(), 1./3.));
        a.insert("sin", (|x| x.sin(), 0.));
        a.insert("cos", (|x| x.cos(), 0.));
        a.insert("tan", (|x| x.tan(), 0.));
        a.insert("asin", (|x| x.asin(), 0.));
        a.insert("acos", (|x| x.acos(), 0.));
        a.insert("atan", (|x| x.atan(), 0.));
        a.insert("fact", (|x| puruspe::gamma(x+1.), 0.));
        a.insert("gamma", (|x| puruspe::gamma(x), 0.));
        a
    };

    static ref UNITS: HashMap<&'static str, Number> = {
        let mut a = HashMap::new();
        // Fundamental units, CGS
        a.insert("cm", Number { q:1., u: Unit::new([1., 0., 0.])});
        a.insert("g", Number { q:1., u: Unit::new([0., 1., 0.])});
        a.insert("s", Number { q:1., u: Unit::new([0., 0., 1.])});
        a.insert("G", Number { q:1., u: Unit::new([-0.5, 0.5, -1.])});
        a.insert("esu", Number { q:1., u: Unit::new([1.5, -0.5, -1.])});
        a.insert("erg", Number { q:1., u: Unit::new([2., 1., -2.])});
        a.insert("dyn", Number { q:1., u: Unit::new([1., 1., -1.])});

        // eV
        a.insert("meV", Number { q:1.60218e-15, u: Unit::new([2., 1., -2.])});
        a.insert("eV", Number { q:1.60218e-12, u: Unit::new([2., 1., -2.])});
        a.insert("keV", Number { q:1.60218e-9, u: Unit::new([2., 1., -2.])});
        a.insert("MeV", Number { q:1.60218e-6, u: Unit::new([2., 1., -2.])});
        a.insert("GeV", Number { q:1.60218e-3, u: Unit::new([2., 1., -2.])});
        a.insert("TeV", Number { q:1.60218, u: Unit::new([2., 1., -2.])});
        a.insert("PeV", Number { q:1.60218e3, u: Unit::new([2., 1., -2.])});
        a.insert("EeV", Number { q:1.60218e6, u: Unit::new([2., 1., -2.])});

        // Length
        a.insert("pc", Number { q:3.086e18, u: Unit::new([1., 0., 0.])});
        a.insert("ly", Number { q:9.461e17, u: Unit::new([1., 0., 0.])});
        a.insert("AU", Number { q:1.496e13, u: Unit::new([1., 0., 0.])});

        // Time
        a.insert("min", Number { q:60., u: Unit::new([0., 0., 1.])});
        a.insert("hr", Number { q:3600., u: Unit::new([0., 0., 1.])});
        a.insert("d", Number { q:3600.*24., u: Unit::new([0., 0., 1.])});
        a.insert("yr", Number { q:3600.*24.*365.25, u: Unit::new([0., 0., 1.])});
        a.insert("kyr", Number { q:3600.*24.*365.25*1000., u: Unit::new([0., 0., 1.])});

        // Bodies
        a.insert("msun", Number { q: 1.989e33, u: Unit::new([0., 1., 0.])});
        a.insert("lsun", Number { q: 3.839e33, u: Unit::new([2., 1., -2.])});
        
        a
    };
}

fn print_help() {
    println!("NUMBERS: ");
    for k in NUMBERS.keys() {
        println!("{}", k);
    }
    println!("");

    println!("CONSTANTS: ");
    for k in CONSTANTS.keys() {
        println!("{}", k);
    }
    println!("");

    println!("FUNCTIONS: ");
    for k in FUNCTIONS.keys() {
        println!("{}", k);
    }
    println!("");

    println!("UNITS: ");
    for k in UNITS.keys() {
        println!("{}", k);
    }
}

fn main() -> rustyline::Result<()> {
    let config = Config::builder()
        .edit_mode(EditMode::Emacs) // or Vi
        .auto_add_history(true)
        .build();

    let mut rl = DefaultEditor::with_config(config)?;

    loop {
        match rl.readline(">>> ") {
            Ok(line) => {
                if line == "" {continue;}
                if line == "exit" {break;}
                if line == "help" {
                    print_help();
                    continue;
                }
                let parsed = match parse(&line) {
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
            Err(rustyline::error::ReadlineError::Eof) => break,
            Err(e) => {
                eprintln!("Error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}
