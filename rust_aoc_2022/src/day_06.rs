use std::path::Path;

use rust_aoc_2022::aoc_lib::day_06::{get_first_packet_index, get_first_message};


fn main() {
    let path = Path::new(r"./res/day_06/day_06.csv");
    println!("first packet at: {:?}", get_first_packet_index(&path));
    println!("first message at: {:?}", get_first_message(&path));
}

