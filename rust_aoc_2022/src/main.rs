use std::path::Path;

use rust_aoc_2022::aoc_lib::elf_food::{find_most_cal_elf, find_3_most_cal_elfs};


fn main() {
    //println!("{:?}", find_most_cal_elf(&Path::new(r"./res/day_1/elf_food_list.csv")));
    println!("{:?}", find_3_most_cal_elfs(&Path::new(r"./res/day_1/elf_food_list.csv")).iter().fold(0, |x,y| x+y));
}
