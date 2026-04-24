#![allow(unused)]

use std::convert::Into;

use Temperature::*;

enum Temperature {
    Celsius(f32),
    Fahrenheit(f32),
}

impl Temperature {
    fn to_celsius(&self) -> Temperature {
        match self {
            Celsius(celsius) => Celsius(*celsius),
            Fahrenheit(fahrenheit) => Celsius(5. * (*fahrenheit - 32.) / 9.),
        }
    }

    fn to_fahrenheit(&self) -> Temperature {
        match self {
            Celsius(celsius) => Fahrenheit(32. + (9. * *celsius / 5.)),
            Fahrenheit(fahrenheit) => Fahrenheit(*fahrenheit),
        }
    }

    fn to_f32(&self) -> f32 {
        match self {
            Celsius(celsius) => *celsius,
            Fahrenheit(fahrenheit) => *fahrenheit,
        }
    }
}

/*
impl Into<f32> for Temperature {
    fn into(self) -> f32 {
        match self {
            Celsius(f) => f.clone(),
            Fahrenheit(f) => f.clone(),
        }
    }
}
 */

fn main() {
    let temp = Celsius(0.0);

    println!("fun fact: 20°C is an integer in celsius and fahrenheit");
    println!(
        "          {:.1}°C = {:.1}°F",
        temp.to_celsius().to_f32(),
        temp.to_fahrenheit().to_f32()
    );
}

#[test]
fn one_degree() {
    let cold = Celsius(1.0);
    assert!((cold.to_fahrenheit().to_f32() - 33.8) < 0.01);
    assert!((cold.to_fahrenheit().to_f32() - 33.8) >= 0.0);
}

#[test]
fn boiling() {
    let hot = Celsius(100.0);
    assert!((hot.to_fahrenheit().to_f32() - 212.0) < 0.01);
    assert!((hot.to_fahrenheit().to_f32() - 212.0) >= 0.0);
}

#[test]
fn freezing() {
    let freezing = Celsius(0.0).to_fahrenheit();

    assert!(freezing.to_celsius().to_f32() < 0.001);
    assert!(freezing.to_celsius().to_f32() > -0.01);
}
