
use std::{path::Path, collections::HashSet};

use rust_aoc_2022::aoc_lib::day_15::{parse_sensor_beacon_positions, get_manhaten_distance, get_all_pos_from_with_dist, SensorBeaconPoss};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let path = Path::new(r"./res/day_15/day_15.csv");
	let positions = parse_sensor_beacon_positions(path)?;
	println!("{:?}", positions);
	let mut covered_positions = HashSet::new();
	let mut becon_sensor_poss = SensorBeaconPoss::new();
	for (sensor, beacon) in positions {
		becon_sensor_poss.beacon_poss.insert(beacon);
		becon_sensor_poss.sensor_poss.insert(sensor);
		let dist = get_manhaten_distance(sensor, beacon);
		let positions = get_all_pos_from_with_dist(sensor, dist);
		for position in positions {
			covered_positions.insert(position);
		}
	}

	let count = covered_positions.iter().filter(|x| x.1 == 2_000_000 && !becon_sensor_poss.beacon_poss.contains(x)).count();

	println!("covered positions: {}", count);

	Ok(())
}
