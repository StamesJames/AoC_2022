use std::path::Path;

use pest::pratt_parser::Op;
use rust_aoc_2022::aoc_lib::day_12::{
    get_path_length_to_position, get_path_length_to_position_from_file, parse_height_map, Direction,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(r"./res/day_12/day_12.csv");
    let heigh_map = parse_height_map(path)?;
    let length_map = get_path_length_to_position(&heigh_map, heigh_map.e_position);

    print_lenghth(&length_map);
    print_dir(&length_map);

    let s_pos = heigh_map.s_position;
    let len_s_pos = &length_map[s_pos.1][s_pos.0];

    println!("path_len: {:?}", len_s_pos);
    let height_map_height = heigh_map.height_map.len();
    let height_map_width = heigh_map.height_map[0].len();
    let mut all_a_pos_length:Vec<usize> = heigh_map
        .height_map
        .iter()
        .flatten()
        .enumerate()
        .filter(|(i, entry)| **entry == 0)
        .map(|(i, entry)| (i % height_map_width, i / height_map_width))
		.filter(|(x,y)| length_map[*y][*x].is_some())
        .map(|(x, y)| match length_map[y][x] {
            Some((len,_)) => len,
            None => panic!("can not be!"),
        }).collect();

	all_a_pos_length.sort();

	println!("a pos length: {:?}", all_a_pos_length);

    Ok(())
}

fn print_lenghth(length_map: &Vec<Vec<Option<(usize, Direction)>>>) {
    println!("");
    for line in length_map {
        for entry in line {
            match entry {
                Some((l, _)) => print!("{l}\t"),
                None => print!(".\t"),
            }
        }
        println!("");
    }
}

fn print_dir(length_map: &Vec<Vec<Option<(usize, Direction)>>>) {
    println!("");
    for line in length_map {
        for entry in line {
            match entry {
                Some((_, d)) => print!("{d}\t"),
                None => print!(".\t"),
            }
        }
        println!("");
    }
}
