
use std::{path::Path};

use rust_aoc_2022::aoc_lib::day_11::parse_monkeys;


fn main() -> Result<(), Box<dyn std::error::Error>> {
	let path = Path::new(r"./res/day_11/day_11.csv");


	let mut monkeys = parse_monkeys(path)?;
	let common_faktor = monkeys.iter().fold(1, |acc, monk| acc * monk.test.divisor);

	for _ in 0..20 {
		for (i, monkey) in (0..monkeys.len()).enumerate() {
			let mut monkey = &mut monkeys[monkey];
			let throws = monkey.take_turn(3, common_faktor);
			for (item, dest_monkey) in throws {
				monkeys[dest_monkey].catch_item(item);
			}
		}
	}

	monkeys.sort_by(|monkey1, monkey2| monkey2.inspection_count.cmp(&monkey1.inspection_count));
	monkeys.iter().for_each(|monkey| println!("Monkey: {}", monkey.inspection_count));
	let monkey_business = monkeys[0].inspection_count * monkeys[1].inspection_count;
	println!("Monkey Business: {monkey_business}");


	let mut monkeys = parse_monkeys(path)?;
	let common_faktor = monkeys.iter().fold(1, |acc, monk| acc * monk.test.divisor);
	for _ in 0..10_000 {
		for (i, monkey) in (0..monkeys.len()).enumerate() {
			let mut monkey = &mut monkeys[monkey];
			let throws = monkey.take_turn(1, common_faktor);
			for (item, dest_monkey) in throws {
				monkeys[dest_monkey].catch_item(item);
			}
		}
	}

	monkeys.sort_by(|monkey1, monkey2| monkey2.inspection_count.cmp(&monkey1.inspection_count));
	monkeys.iter().for_each(|monkey| println!("Monkey: {}", monkey.inspection_count));
	let monkey_business = monkeys[0].inspection_count * monkeys[1].inspection_count;

	println!("Monkey Business: {monkey_business}");
	Ok(())
}
