use rust_aoc_2022::aoc_lib::utils::fetch_res_and_save_to_file;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];

    return fetch_res_and_save_to_file(day);
}
