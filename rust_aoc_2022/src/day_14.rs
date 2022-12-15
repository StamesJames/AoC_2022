
use std::path::Path;

use rust_aoc_2022::aoc_lib::day_14::parse_stone_structures;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let path = Path::new(r"./res/day_14/day_14_test.csv");
	let mut cave = parse_stone_structures(path)?;
	cave.print_cave();
	for _ in 0..20 {
		cave.throw_sand_at((500,0));
	}

	cave.print_cave();
	Ok(())
}
