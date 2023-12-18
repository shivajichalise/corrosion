use std::io::{self, Write};

enum PatternVerticleType {
    TopToBottom,
    BottomToTop,
}

enum PatternHorizontalType {
    LeftToRight,
    RightToLeft,
}

fn take_length_input() -> u32 {
    let mut input = String::new();

    print!("Give the length of the pattern: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input. :(");

    let input: u32 = match input.trim().parse() {
        Ok(i) => i,
        Err(_) => {
            println!();
            println!("-----------------------");
            println!("Please give an integer.");
            println!("-----------------------");
            println!();
            take_length_input()
        }
    };

    input
}

fn take_verticle_type_input() -> PatternVerticleType {
    let mut input = String::new();

    println!("Please choose one option.");
    println!("1. Top to bottom");
    println!("2. Bottom to top");
    print!("Enter either 1 or 2: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input. :(");

    let input: PatternVerticleType = match input.trim().parse() {
        Ok(i) => match i {
            1 => PatternVerticleType::TopToBottom,
            2 => PatternVerticleType::BottomToTop,
            _ => {
                println!();
                println!("***************************");
                println!("Please enter either 1 or 2.");
                println!("***************************");
                println!();
                take_verticle_type_input()
            }
        },
        Err(_) => {
            println!();
            println!("-----------------------");
            println!("Please give an integer.");
            println!("-----------------------");
            println!();
            take_verticle_type_input()
        }
    };

    input
}

fn take_horizontal_type_input() -> PatternHorizontalType {
    let mut input = String::new();

    println!("Please choose one option.");
    println!("1. Left to right");
    println!("2. Right to left");
    print!("Enter either 1 or 2: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input. :(");

    let input: PatternHorizontalType = match input.trim().parse() {
        Ok(i) => match i {
            1 => PatternHorizontalType::LeftToRight,
            2 => PatternHorizontalType::RightToLeft,
            _ => {
                println!();
                println!("***************************");
                println!("Please enter either 1 or 2.");
                println!("***************************");
                println!();
                take_horizontal_type_input()
            }
        },
        Err(_) => {
            println!();
            println!("-----------------------");
            println!("Please give an integer.");
            println!("-----------------------");
            println!();
            take_horizontal_type_input()
        }
    };

    input
}

fn print_top_to_bottom_pattern(length: u32, horizontal_type: PatternHorizontalType) {
    for i in 1..=length {
        match horizontal_type {
            PatternHorizontalType::LeftToRight => {
                print_left_to_right(i, length, PatternVerticleType::TopToBottom)
            }
            PatternHorizontalType::RightToLeft => {
                print_right_to_left(i, length, PatternVerticleType::TopToBottom)
            }
        }
    }
}

fn print_bottom_to_top(length: u32, horizontal_type: PatternHorizontalType) {
    for i in 1..=length {
        match horizontal_type {
            PatternHorizontalType::LeftToRight => {
                print_left_to_right(i, length, PatternVerticleType::BottomToTop)
            }
            PatternHorizontalType::RightToLeft => {
                print_right_to_left(i, length, PatternVerticleType::BottomToTop)
            }
        }
    }
}

fn print_left_to_right(i: u32, length: u32, verticle_type: PatternVerticleType) {
    match verticle_type {
        PatternVerticleType::TopToBottom => {
            for _ in 1..=i {
                print!("*");
            }
        }
        PatternVerticleType::BottomToTop => {
            for _ in i..=length {
                print!("*");
            }
        }
    }
    println!();
}

fn print_right_to_left(i: u32, length: u32, verticle_type: PatternVerticleType) {
    match verticle_type {
        PatternVerticleType::TopToBottom => {
            for _ in i..length {
                print!(" ");
            }

            for _ in 1..=i {
                print!("*");
            }
        }
        PatternVerticleType::BottomToTop => {
            for _ in 1..i {
                print!(" ");
            }

            for _ in i..=length {
                print!("*");
            }
        }
    }
    println!();
}

fn print_pattern(
    length: u32,
    verticle_type: PatternVerticleType,
    horizontal_type: PatternHorizontalType,
) {
    match verticle_type {
        PatternVerticleType::TopToBottom => print_top_to_bottom_pattern(length, horizontal_type),
        PatternVerticleType::BottomToTop => print_bottom_to_top(length, horizontal_type),
    }
}

fn main() {
    println!("-----------------------");
    println!("Print Pattern");
    println!("-----------------------");

    let length = take_length_input();
    println!();

    let verticle_type = take_verticle_type_input();
    println!();

    let horizontal_type = take_horizontal_type_input();
    println!();

    print_pattern(length, verticle_type, horizontal_type);
}
