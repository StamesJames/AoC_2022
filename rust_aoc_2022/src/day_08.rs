use std::{path::{self, Path}, fmt::Debug};

use nalgebra::DMatrix;
use rust_aoc_2022::aoc_lib::day_08::{parse_matrix, get_visibility_matrix, get_looking_distance_matrix};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(r"./res/day_08/day_08.csv");
    let matrix = parse_matrix(path)?;
    println!("MATRIX:");
    print_matrix(&matrix);
    
    let visibility_matrix = get_visibility_matrix(&matrix);
    println!("VISIBILITY MATRIX:");
    print_matrix(&visibility_matrix);
    let visible_count = visibility_matrix.sum();
    println!("visible_count: {visible_count}");
    let looking_dist_matrix = get_looking_distance_matrix(&matrix);
    println!("LOOKING DIST MATRIX:");
    print_matrix(&looking_dist_matrix);
    
    let max_scenic_score = looking_dist_matrix.iter().map(|(x,y,z,v)| x * y * z *v).max();
    println!("max_scenic_score: {:?}", max_scenic_score);

    Ok(())
}


fn print_matrix<T>(matrix: &DMatrix<T>) 
where
    T: Debug
{
    for row in matrix.row_iter() {
        for elem in row.iter() {
            print!("{:?}", elem);
        }
        println!("");
    }
    println!("");
}