use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, Write};

fn handle_input(input: &str) -> Option<f64> {
    lazy_static! {
        static ref ADD_RE: Regex = Regex::new(r"add (\d+(?:\.\d+)?) and (\d+(?:\.\d+)?)").unwrap();
        static ref SUBTRACT_RE: Regex =
            Regex::new(r"subtract (\d+(?:\.\d+)?) from (\d+(?:\.\d+)?)").unwrap();
        static ref MULTIPLY_RE: Regex =
            Regex::new(r"multiply (\d+(?:\.\d+)?) by (\d+(?:\.\d+)?)").unwrap();
        static ref DIVIDE_RE: Regex =
            Regex::new(r"divide (\d+(?:\.\d+)?) by (\d+(?:\.\d+)?)").unwrap();
        static ref PERCENT_RE: Regex =
            Regex::new(r"what is (\d+(?:\.\d+)?)% of (\d+(?:\.\d+)?)").unwrap();
        static ref SQRT_RE: Regex = Regex::new(r"(square root|sqrt) of (\d+(?:\.\d+)?)").unwrap();
        static ref POWER_RE: Regex =
            Regex::new(r"what is (\d+(?:\.\d+)?) to the power of (\d+(?:\.\d+)?)").unwrap();
        static ref KM_TO_MI_RE: Regex = Regex::new(r"convert (\d+(?:\.\d+)?) km to miles").unwrap();
        static ref KG_TO_LB_RE: Regex =
            Regex::new(r"convert (\d+(?:\.\d+)?) kg to pounds").unwrap();
    }

    if let Some(caps) = ADD_RE.captures(input) {
        let a: f64 = caps[1].parse().ok()?;
        let b: f64 = caps[2].parse().ok()?;
        return Some(a + b);
    }

    if let Some(caps) = MULTIPLY_RE.captures(input) {
        let a: f64 = caps[1].parse().ok()?;
        let b: f64 = caps[2].parse().ok()?;
        return Some(a * b);
    }

    if let Some(caps) = PERCENT_RE.captures(input) {
        let percent: f64 = caps[1].parse().ok()?;
        let base: f64 = caps[2].parse().ok()?;
        return Some((percent / 100.0) * base);
    }

    if let Some(caps) = SUBTRACT_RE.captures(input) {
        let a: f64 = caps[2].parse().ok()?;
        let b: f64 = caps[1].parse().ok()?;
        return Some(a - b);
    }

    if let Some(caps) = DIVIDE_RE.captures(input) {
        let a: f64 = caps[1].parse().ok()?;
        let b: f64 = caps[2].parse().ok()?;
        if b == 0.0 {
            println!("Error: Division by zero is not allowed.");
            return None;
        }
        return Some(a / b);
    }

    if let Some(caps) = SQRT_RE.captures(input) {
        let a: f64 = caps[2].parse().ok()?;
        if a < 0.0 {
            println!("Error: Cannot take square root of a negative number.");
            return None;
        }
        return Some(a.sqrt());
    }
    if let Some(caps) = POWER_RE.captures(input) {
        let base: f64 = caps[1].parse().ok()?;
        let exponent: f64 = caps[2].parse().ok()?;
        return Some(base.powf(exponent));
    }
    if let Some(caps) = KM_TO_MI_RE.captures(input) {
        let km: f64 = caps[1].parse().ok()?;
        return Some(km * 0.621371);
    }

    if let Some(caps) = KG_TO_LB_RE.captures(input) {
        let kg: f64 = caps[1].parse().ok()?;
        return Some(kg * 2.20462);
    }

    None
}

fn main() {
    println!("Welcome to the Smart CLI Calculator!");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim().to_lowercase();

        if input == "exit" || input == "quit" {
            println!("Goodbye!");
            break;
        }

        match handle_input(&input) {
            Some(result) => println!("= {}", result),
            None => println!("Sorry, I couldn't understand that."),
        }
    }
}
