
use std::path::Path;

use rust_aoc_2022::aoc_lib::day_10::{get_register_list_from_file, get_img_from_file};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let path = Path::new(r"./res/day_10/day_10.csv");
	let register_list = get_register_list_from_file(path)?;
	let mut sum = 0; 
	for (cycle, register) in register_list.iter().enumerate().skip(19).step_by(40) {
		println!("cycle: {cycle} register: {register}");
		sum += ((cycle + 1) as isize) * register;
	}
	println!("sum: {sum}");

	println!("{:?}", register_list);
	let img = get_img_from_file(path)?;
	for chunck in img.chunks(40) {
		for bit in chunck {
			print!("{}", if *bit {"#"} else {" "});
		}
		println!("");
	}

	Ok(())
}
