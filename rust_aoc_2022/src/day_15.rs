
use std::{path::Path, collections::HashSet};

use rust_aoc_2022::aoc_lib::day_15::{parse_sensor_beacon_positions, get_manhaten_distance, get_all_pos_from_with_dist, SensorBeaconPoss};

fn main() -> Result<(), Box<dyn std::error::Error>> {


	let path = Path::new(r"./res/day_15/day_15.csv");
	let positions = parse_sensor_beacon_positions(path)?;
	let sensor_beacon_poss = SensorBeaconPoss::from_sensor_beacon_pairs(positions);
	// println!("beginn");
	// let covered_in_row_count = sensor_beacon_poss.get_covered_or_beacon_in_row(2_000_000);
	// println!("covered in row: {}", covered_in_row_count);

	let uncoverd = sensor_beacon_poss.get_first_uncovered();
	println!("{:?}", uncoverd);

	Ok(())
}
