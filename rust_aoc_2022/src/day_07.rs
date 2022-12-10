use std::path::Path;

use rust_aoc_2022::aoc_lib::day_07::FileSystem;



fn main() -> Result<(), Box<dyn std::error::Error>>{
    let path = Path::new(r"./res/day_07/day_07.csv");
    let file_system = FileSystem::from_file(path)?;

    file_system.print();

    let fu = file_system.get_dir_sizes_below(100_000);
    println!("{:?}", fu);
    println!("sum of all below 100_000: {}", fu.iter().sum::<usize>());

    let disk_space = 70_000_000;
    let update_size = 30_000_000;
    let filse_system_size = file_system.total_size();
    let free_space = disk_space - filse_system_size;
    let space_to_free = update_size - free_space;

    let fu = *file_system.get_dir_sizes_above(space_to_free).iter().min().unwrap();
    println!("the smalest dir above 30_000_000 has size: {}", fu);

    Ok(())
}