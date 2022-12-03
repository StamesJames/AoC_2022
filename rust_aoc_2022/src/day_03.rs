use std::path::Path;

use rust_aoc_2022::aoc_lib::day_03::{char_to_priority, get_priority_sum, get_badge_priority_sum};



fn main(){
    let path = Path::new("./res/day_03/rucksack_list.csv");
    println!("sum of prios is: {}", get_priority_sum(&path));
    println!("sum of badge Priority: {}", get_badge_priority_sum(&path));
}