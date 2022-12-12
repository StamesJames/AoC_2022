use std::path::Path;
use rust_aoc_2022::aoc_lib::day_09::make_moves_from_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(r"./res/day_09/day_09.csv");
    let head_tail_pos = make_moves_from_file(&path, 2)?;
    let positions = &head_tail_pos.head_tail_series[0];
    let positions = &positions.tail_pos_set;
    println!("{:?}", positions.len());

    let head_tail_pos = make_moves_from_file(&path, 10)?;
    let positions = &head_tail_pos.head_tail_series[0];
    let positions = &positions.tail_pos_set;
    println!("{:?}", positions.len());
    Ok(())
}




