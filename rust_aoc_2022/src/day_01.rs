use std::path::Path;

use rust_aoc_2022::aoc_lib::day_01::{find_3_most_cal_elfs, find_most_cal_elf};

fn main() {
    day_1();
}

fn day_1() {
    let path = Path::new(r"./res/day_01/day_01.csv");
    let (calories, elf_index) = find_most_cal_elf(&path);
    println!("Elf number {elf_index} has with {calories} the most calories");
    let v @ [elf_1, elf_2, elf_3] =
        find_3_most_cal_elfs(&path);
    let sum: usize = v.iter().sum();
    println!("the most calories are {elf_1}, {elf_2}, {elf_3} with a toatl of {sum}");
}
