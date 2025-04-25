use std::collections::HashSet;
use shift_maker::util;
use util::partial_shuffle;

fn main() {
    let indices: HashSet<usize> = (0..8).collect();

    let mut names = vec!["a", "b", "c", "d", "e", "f", "g", "h"];

    partial_shuffle(&mut names, indices);

    println!("{:?}", names);
}