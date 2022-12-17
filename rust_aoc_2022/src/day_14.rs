

use std::{path::Path, io::{self, Write}};

use rust_aoc_2022::aoc_lib::{day_14::{parse_stone_structures,}, utils::GenDynResult};


fn main() -> GenDynResult<()> {
	let path = Path::new(r"./res/day_14/day_14.csv");
	let mut cave = parse_stone_structures(path)?;

	cave.print_cave();

	let mut throw_count = 0;
	while let Some(landing) = cave.throw_sand_from((500,0)) {
		throw_count += 1;
	}

	cave.print_cave();
	println!("throw count: {throw_count}");

	let mut cave = parse_stone_structures(path)?;
	let floor_y = cave.get_max_y() +2;

	let mut throw_count = 0;
	loop {
		let landing = cave.throw_sand_from_with_floor((500,0), floor_y).unwrap();
		throw_count += 1;
		if landing == (500,0){
			break;
		}
	}
	cave.print_cave();
	println!("trhrow cont {}", throw_count);

	Ok(())
}


