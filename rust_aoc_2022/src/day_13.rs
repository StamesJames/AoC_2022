use std::path::Path;

use rust_aoc_2022::aoc_lib::day_13::{parse_package_data, PacketData};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(r"./res/day_13/day_13.csv");
    let package_data = parse_package_data(path)?;
    println!("{:?}", package_data);
    let sum: usize = package_data
        .iter()
        .enumerate()
        .map(|(i, (first, second))| if first <= second { i + 1 } else { 0 })
        .sum();

    println!("{sum}");

    let mut package_list: Vec<PacketData> = package_data
        .into_iter()
        .flat_map(|(first, second)| [first, second].into_iter())
        .collect();
    let div_1 = PacketData::List(vec![PacketData::List(vec![PacketData::Int(2)])]);
    let div_2 = PacketData::List(vec![PacketData::List(vec![PacketData::Int(6)])]);
    package_list.push(div_1);
    package_list.push(div_2);
    package_list.sort();

    for package in package_list.iter() {
        println!("{:?}", package);
    }
    let div_1 = PacketData::List(vec![PacketData::List(vec![PacketData::Int(2)])]);
    let div_2 = PacketData::List(vec![PacketData::List(vec![PacketData::Int(6)])]);
    let result: usize = package_list
        .into_iter()
        .enumerate()
        .filter(|(_, pack)| *pack == div_1 || *pack == div_2)
        .map(|tup| tup.0 + 1)
        .product();

    println!("{result}");

    Ok(())
}
