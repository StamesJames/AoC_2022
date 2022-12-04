use std::path::Path;

use rust_aoc_2022::aoc_lib::day_02::{get_rps_score, get_rps_score_with_endings};

fn main() {
    day_2();
}

fn day_2() {
    let path = Path::new(r"./res/day_2/day_02.csv");
    let score = get_rps_score(&path);
    println!("The Score is {score}");
    let score = get_rps_score_with_endings(&path);
    println!("The Score with ending encoding is {score}");
}
