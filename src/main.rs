use std::{env, io};
use shift_maker::*;
use util::input_time;
use time::{Time, Duration};
use rand::seq::SliceRandom;
use rand::rng;

fn print_fixed_shift_list() {
    let mut input = String::new();

    println!("Enter start time (HH or HH:MM):");

    let start_time_str = input_time().expect("Invalid start time");

    let start_time = Time::from_string(&start_time_str).expect("Invalid start time");

    println!("Enter end time (HH or HH:MM):");

    let end_time_str = input_time().expect("Invalid end time");

    let end_time = Time::from_string(&end_time_str).expect("Invalid end time");

    println!("Enter shift length (HH or HH:MM):");

    let shift_length_str = input_time().expect("Invalid shift length");

    let shift_length = Duration::from_string(&shift_length_str).expect("Invalid shift length");

    println!("Randomize order? (y/N):");

    let is_random:bool;

    io::stdin().read_line(&mut input)
        .expect("Input error");

    match input.trim().to_lowercase().as_str() {
        "" => is_random = false,
        "n" => is_random = false,
        "y" => is_random = true,
        _ => panic!("Invalid input"),
    }

    input.clear();

    println!("Enter names separated by whitespace:");

    io::stdin().read_line(&mut input)
        .expect("Input error");

    let mut names: Vec<&str> = input.split_whitespace().collect();

    if is_random {
        let mut rng = rng();
        names.shuffle(&mut rng);
    }

    let shift_list = interval_fixed_shift_list(&names, shift_length, start_time, end_time);

    for (name, start, end) in shift_list {
        println!("{}-{} {}", start, end, name);
    }
}

fn print_shift_list() {
    let mut input = String::new();

    println!("Enter start time (HH or HH:MM):");

    let start_time_str = input_time().expect("Invalid start time");

    let start_time = Time::from_string(&start_time_str).expect("Invalid start time");

    println!("Enter end time (HH or HH:MM):");

    let end_time_str = input_time().expect("Invalid end time");

    let end_time = Time::from_string(&end_time_str).expect("Invalid end time");

    println!("Randomize order? (y/N):");

    let is_random:bool;

    io::stdin().read_line(&mut input)
        .expect("Input error");

    match input.trim().to_lowercase().as_str() {
        "" => is_random = false,
        "n" => is_random = false,
        "y" => is_random = true,
        _ => panic!("Invalid input"),
    }

    input.clear();

    println!("Enter names separated by whitespace:");

    io::stdin().read_line(&mut input)
        .expect("Input error");

    let mut names: Vec<&str> = input.split_whitespace().collect();

    if is_random {
        let mut rng = rng();
        names.shuffle(&mut rng);
    }

    let shift_list = interval_var_shift_list(&names, start_time, end_time);

    print!("{}", shift_list_to_string(&shift_list));
}

fn main() {


    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--fixed-shift" {
        print_fixed_shift_list();
    } else {
        print_shift_list();
    }
}

