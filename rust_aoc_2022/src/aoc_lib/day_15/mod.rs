use std::{fs::File, io::{BufReader, BufRead}, path::Path, collections::{HashSet, HashMap}, hash::Hash};

use rayon::result;
use regex::Regex;

use super::utils::{GenDynResult, EmptyOptionError};

pub fn get_manhaten_distance((x1,y1):(isize, isize), (x2,y2):(isize, isize)) -> usize {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

pub fn get_all_pos_from_in_dist_in_row((x,y):(isize, isize), dist:usize, row_y:isize) -> HashSet<(isize, isize)>{
    let mut result = HashSet::new();
    let dist_to_row = y.abs_diff(row_y);
    for i in 0..=(dist.saturating_sub(dist_to_row)) as isize{
        result.insert((x+i,row_y));
        result.insert((x-i,row_y));
    }

    result
}

pub fn add_all_pos_from_in_dist_in_row((x,y):(isize, isize), dist:usize, row_y:isize, set:&mut HashSet<(isize, isize)>){
    let dist_to_row = y.abs_diff(row_y);
    for i in 0..=(dist.saturating_sub(dist_to_row)) as isize{
        set.insert((x+i,row_y));
        set.insert((x-i,row_y));
    }
}

pub fn get_all_pos_from_with_dist((x,y):(isize, isize), dist: usize) -> HashSet<(isize, isize)> {
    let mut result = HashSet::new();

    for i in 0..=dist as isize {
        for d_x in 0..=i {
            result.insert((x + d_x,y + (i-d_x)));
            result.insert((x - d_x,y + (i-d_x)));
            result.insert((x + d_x,y - (i-d_x)));
            result.insert((x - d_x,y - (i-d_x)));
        }
    }

    result
}


pub fn parse_sensor_beacon_positions(path:&Path) -> GenDynResult<Vec<((isize, isize), (isize, isize))>> {
    let mut result = Vec::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines(){
        let line = line?;
        let mut line = line.trim().split(":");

        let sensor_str = line.next().ok_or(EmptyOptionError)?;
        let re = Regex::new(r"x=(-?\d*), y=(-?\d*)").unwrap();
        let sensor_cap = re.captures_iter(sensor_str).next().unwrap();
        println!("sen x: {}, sen y: {}", &sensor_cap[1], &sensor_cap[2]);
        let sensor_pos = (sensor_cap[1].parse()?, sensor_cap[2].parse()?);

        let beacon_str = line.next().ok_or(EmptyOptionError)?;
        let beacon_cap = re.captures_iter(beacon_str).next().unwrap();
        println!("bea x: {}, bea y: {}", &beacon_cap[1], &beacon_cap[2]);
        let beacon_pos = (beacon_cap[1].parse()?, beacon_cap[2].parse()?);

        result.push((sensor_pos, beacon_pos));
    }


    Ok(result)
}

pub struct SensorBeaconPoss{
    sensor_poss: HashSet<(isize, isize)>,
    beacon_poss: HashSet<(isize, isize)>,
    sensor_beacon_pairs: HashMap<(isize, isize), (isize, isize)>,
    sensor_rows: HashMap<isize, HashSet<isize>>,
    beacon_rows: HashMap<isize, HashSet<isize>>,
    sensor_radius: HashMap<(isize,isize), usize>,
}

impl SensorBeaconPoss {
    pub fn new() -> Self { Self { sensor_poss: HashSet::new(), beacon_poss: HashSet::new(), sensor_beacon_pairs: HashMap::new(), sensor_rows: HashMap::new(), beacon_rows: HashMap::new(), sensor_radius: HashMap::new()} }

    pub fn from_sensor_beacon_pairs(sensor_beacon_pairs_vec: Vec<((isize,isize),(isize,isize))>) -> Self {
        let mut sensor_poss = HashSet::new();
        let mut beacon_poss = HashSet::new();
        let mut sensor_beacon_pairs = HashMap::new();
        let mut sensor_rows = HashMap::new();
        let mut beacon_rows = HashMap::new();
        let mut sensor_radius = HashMap::new();

        for (sensor, beacon) in sensor_beacon_pairs_vec {
            sensor_poss.insert(sensor);
            beacon_poss.insert(beacon);
            sensor_beacon_pairs.insert(sensor, beacon);
            if !sensor_rows.contains_key(&sensor.1) {
                sensor_rows.insert(sensor.1, HashSet::new());
            }
            sensor_rows.get_mut(&sensor.1).unwrap().insert(sensor.0);
            if !beacon_rows.contains_key(&beacon.1) {
                beacon_rows.insert(beacon.1, HashSet::new());
            }
            beacon_rows.get_mut(&beacon.1).unwrap().insert(beacon.0);
            sensor_radius.insert(sensor, get_manhaten_distance(sensor, beacon));

        }

        Self { sensor_poss, beacon_poss, sensor_beacon_pairs, sensor_rows, beacon_rows, sensor_radius }
    }

    pub fn get_covered_in_row(&self, row_y:isize) -> usize{
        let mut covered_in_row: HashSet<(isize,isize)> = HashSet::new();
        for (sensor,beacon) in self.sensor_beacon_pairs.iter() {
            let dist = get_manhaten_distance(*sensor, *beacon);
            if dist > sensor.1.abs_diff(row_y) {
                covered_in_row.extend(get_all_pos_from_in_dist_in_row(*sensor, dist, row_y).iter().filter(|x| !self.beacon_poss.contains(x)));
            }
        }

        covered_in_row.len()
    }
    

    pub fn get_covered_or_beacon_in_row(&self, row_y:isize) -> usize{
        let mut covered_in_row: HashSet<(isize,isize)> = HashSet::new();
        for (sensor,beacon) in self.sensor_beacon_pairs.iter() {
            let dist = get_manhaten_distance(*sensor, *beacon);
            if dist > sensor.1.abs_diff(row_y) {
                add_all_pos_from_in_dist_in_row(*sensor, dist, row_y, &mut covered_in_row);
            }
        }

        covered_in_row.len()
    }

    pub fn get_first_uncovered(&self) -> (isize, isize) {
        let result = (-1,-1);
        for y in 0..=4_000_000 {
            let mut x = 0;
            while x <= 4_000_000 {
                let (covered, dist) = self.is_covered((x,y));
                if !covered {
                    return (x,y);
                }
                x += (dist as isize).max(1);
            }
        }

        result
    }

    pub fn is_covered(&self, (x,y):(isize, isize)) -> (bool, usize) {
        let mut covered = false;
        let mut max_d = 0;
        for (sensor, beacon) in self.sensor_beacon_pairs.iter() {
            let dist = get_manhaten_distance(*sensor, (x,y));
            let sensor_radius = self.sensor_radius[sensor];
            if sensor_radius >= dist {
                covered = true;
                max_d = max_d.max(sensor_radius - dist);
            }
        }

        (covered, max_d)
    }


}