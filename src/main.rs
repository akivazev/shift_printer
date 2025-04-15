use std::{env, io};
use std::collections::HashSet;
use shift_maker::*;
use util::input_time;
use time::{Time, Duration};
use rand::seq::SliceRandom;
use rand::rng;

#[derive(Debug)]
struct ConfigState {
    is_fixed_shift_length: bool,
    randomize: bool,
    start_time: Time,
    end_time: Time,
    shift_length: Duration
}

#[derive(Hash, PartialEq, Eq, Debug)]
enum ConfigOptions {
    FixedShiftLength,
    Randomize,
}

fn generic_print_shift_list(options: HashSet<ConfigOptions>) {

    // program state control
    let mut state = ConfigState {
        is_fixed_shift_length: options.contains(&ConfigOptions::FixedShiftLength),
        randomize: options.contains(&ConfigOptions::Randomize),
        start_time: Default::default(),
        end_time: Default::default(),
        shift_length: Default::default(),
    };

    // start time input
    println!("Enter start time (HH or HH:MM):");

    let start_time_str = input_time().expect("Invalid start time");
    state.start_time = Time::from_string(&start_time_str).expect("Invalid start time");

    // end time input
    println!("Enter end time (HH or HH:MM):");

    let end_time_str = input_time().expect("Invalid end time");
    state.end_time = Time::from_string(&end_time_str).expect("Invalid end time");

    if state.is_fixed_shift_length {

        // shift duration input
        println!("Enter shift length (HH or HH:MM):");

        let shift_length_str = input_time().expect("Invalid shift length");
        state.shift_length = Duration::from_string(&shift_length_str).expect("Invalid shift length");
    }

    let mut input = String::new();

    // ask user whether to randomize
    if !state.randomize {
        println!("Randomize order? (y/N):");

        io::stdin().read_line(&mut input)
            .expect("Input error");

        match input.trim().to_lowercase().as_str() {
            "" => state.randomize = false,
            "n" => state.randomize = false,
            "y" => state.randomize = true,
            _ => panic!("Invalid input"),
        }
        input.clear();
    }

    // input names and convert to vector
    println!("Enter names separated by whitespace:");

    io::stdin().read_line(&mut input)
        .expect("Input error");

    let mut names: Vec<&str> = input.split_whitespace().collect();

    // shuffle names for randomization case
    if state.randomize {
        let mut rng = rng();
        names.shuffle(&mut rng);
    }

    // create shift list
    let shift_list = if state.is_fixed_shift_length {
        interval_fixed_shift_list(&names, state.shift_length, state.start_time, state.end_time)
    } else {
        interval_var_shift_list(&names, state.start_time, state.end_time)
    };

    // convert shift list to readable string and print
    print!("{}", shift_list_to_string(&shift_list));

}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut options: HashSet<ConfigOptions> = HashSet::new();

    // check for option argument tags
    if args.contains(&String::from("--fixed-shift")) {
        options.insert(ConfigOptions::FixedShiftLength);
    }
    if args.contains(&String::from("--randomize")) {
        options.insert(ConfigOptions::Randomize);
    }

    generic_print_shift_list(options);
}

