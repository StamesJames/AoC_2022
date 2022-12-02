use std::path::Path;

use rust_aoc_2022::aoc_lib::{day_1::{find_most_cal_elf, find_3_most_cal_elfs}, day_2::{get_rps_score, get_rps_score_with_endings}};


fn main() {
    day_2();
}


fn day_2(){
    let score = get_rps_score(&Path::new(r"./res/day_2/rock_paper_scissors_guide.csv"));
    println!("The Score is {score}");
    let score = get_rps_score_with_endings(&Path::new(r"./res/day_2/rock_paper_scissors_guide.csv"));
    println!("The Score with ending encoding is {score}");
}

fn day_1(){
    let (calories, elf_index) = find_most_cal_elf(&Path::new(r"./res/day_1/elf_food_list.csv"));
    println!("Elf number {elf_index} has with {calories} the most calories");
    let v@[elf_1, elf_2, elf_3] = find_3_most_cal_elfs(&Path::new(r"./res/day_1/elf_food_list.csv"));
    let sum:usize = v.iter().sum();
    println!("the most calories are {elf_1}, {elf_2}, {elf_3} with a toatl of {sum}");
}

