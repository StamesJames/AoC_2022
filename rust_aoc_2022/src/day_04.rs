use std::path::Path;

use rust_aoc_2022::aoc_lib::day_04::{get_containing_count, get_overlapping_count};

fn main(){
    let path = Path::new("./res/day_04/day_04.csv");
    println!("Containing count: {:?}", get_containing_count(path));
    println!("Overlapping count: {:?}", get_overlapping_count(path))
}