
use std::{path::Path};

use rust_aoc_2022::aoc_lib::day_11::parse_monkeys;


fn main() -> Result<(), Box<dyn std::error::Error>> {
	let path = Path::new(r"./res/day_11/day_11.csv");
	let mut monkeys = parse_monkeys(path)?;

	for (i, monkey) in (0..monkeys.len()).enumerate() {
		let mut monkey = &mut monkeys[monkey];
		let throws = monkey.take_turn();
		for (item, dest_monkey) in throws {
			monkeys[dest_monkey].catch_item(item);
		}
	}

	Ok(())
}
