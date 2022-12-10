use std::path::Path;

use rust_aoc_2022::aoc_lib::day_05::tower_moves_parse;



fn main() -> Result<(), Box<dyn std::error::Error>>{
    let path = Path::new("./res/day_05/day_05.csv");

    let (towers, moves) = tower_moves_parse(path)?;

    let mut towers_1 = towers.clone();
    let mut towers_2 = towers.clone();
    moves.iter().for_each(|m| m.execute_on(&mut towers_1));
    moves.iter().for_each(|m| m.execute_9001_on(&mut towers_2));
    println!("towers_1");
    towers_1.print_all_top();
    println!("towers_2");
    towers_2.print_all_top();
    Ok(())
}
