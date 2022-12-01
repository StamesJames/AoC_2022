use std::path::Path;

use rust_aoc_2022::aoc_lib::elf_food::{find_most_cal_elf, find_3_most_cal_elfs};


fn main() {
    day_1();
}


fn day_1(){
    let (calories, elf_index) = find_most_cal_elf(&Path::new(r"./res/day_1/elf_food_list.csv"));
    println!("Elf number {elf_index} has with {calories} the most calories");
    let v@[elf_1, elf_2, elf_3] = find_3_most_cal_elfs(&Path::new(r"./res/day_1/elf_food_list.csv"));
    let sum:usize = v.iter().sum();
    println!("the most calories are {elf_1}, {elf_2}, {elf_3} with a toatl of {sum}");
}