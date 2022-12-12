use std::{fs::File, io::{BufReader, BufRead}, path::Path, vec};

use nalgebra::{DMatrix, DVector, Matrix1xX};


pub fn get_visible_tree_count(path: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    let matrix = parse_matrix(path)?;
    return Ok(get_visible_tree_count_matrix(matrix));
}

pub fn get_visible_tree_count_matrix(matrix: DMatrix<i32>) -> usize {
    todo!()
}


pub fn get_looking_distance_matrix(matrix: &DMatrix<i32>) -> DMatrix<(usize,usize,usize,usize)> {
    let mut looking_distance_matrix = DMatrix::from_element(matrix.nrows(), matrix.ncols(), (0,0,0,0));
    

    for (row_num, row) in matrix.row_iter().enumerate(){
        let mut row_vec = Vec::new();
        for (col_num, elem) in row.iter().enumerate() {
            let distance = get_looking_distance(*elem, &row_vec);
            row_vec.push(*elem);
            looking_distance_matrix[(row_num, col_num)].0 = distance;
        }
        
        let mut row_vec = Vec::new();
        for (col_num, elem) in row.iter().enumerate().rev() {
            let distance = get_looking_distance(*elem, &row_vec);
            row_vec.push(*elem);
            looking_distance_matrix[(row_num, col_num)].1 = distance;
        }
    }

    for (col_num, col) in matrix.column_iter().enumerate() {
        let mut col_vec = Vec::new();
        
        for (row_num, elem) in col.iter().enumerate(){
            let distance = get_looking_distance(*elem, &col_vec);
            col_vec.push(*elem);
            looking_distance_matrix[(row_num, col_num)].2 = distance;
        }
        
        let mut col_vec = Vec::new();
        for (row_num, elem) in col.iter().enumerate().rev(){
            let distance = get_looking_distance(*elem, &col_vec);
            col_vec.push(*elem);
            looking_distance_matrix[(row_num, col_num)].3 = distance;
        }
    }
    
    return looking_distance_matrix;
}

pub fn get_looking_distance(n: i32, trees: &Vec<i32>)-> usize{
    let mut distance = 0;
    
    for tree in trees.iter().rev() {
        distance += 1;
        if tree >= &n {
            break;
        }
    }

    distance
}

pub fn get_visibility_matrix(matrix: &DMatrix<i32>) -> DMatrix<i32> {
    let mut visibilitiy_matrix = DMatrix::from_element(matrix.nrows(), matrix.ncols(), 0);
    
    let mut current_max = -1;

    for (row_num, row) in matrix.row_iter().enumerate(){
        for (col_num, elem) in row.iter().enumerate() {
            if elem > &current_max {
                visibilitiy_matrix[(row_num, col_num)] = 1;
                current_max = *elem;
            }
        }
        current_max = -1;
        for (col_num, elem) in row.iter().enumerate().rev() {
            if elem > &current_max {
                visibilitiy_matrix[(row_num, col_num)] = 1;
                current_max = *elem;
            }
        }
        current_max = -1;
    }

    for (col_num, col) in matrix.column_iter().enumerate() {
        for (row_num, elem) in col.iter().enumerate(){
            if elem > &current_max {
                visibilitiy_matrix[(row_num, col_num)] = 1;
                current_max = *elem;
            }
        }
        current_max = -1;
        for (row_num, elem) in col.iter().enumerate().rev(){
            if elem > &current_max {
                visibilitiy_matrix[(row_num, col_num)] = 1;
                current_max = *elem;
            }
        }
        current_max = -1;
    }
    
    return visibilitiy_matrix;
}

pub fn parse_matrix(path: &Path) -> Result<DMatrix<i32>, Box<dyn std::error::Error>> {
    let mut matrix: Vec<Vec<_>> = Vec::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines(){
        let line = line?;
        let line = line.trim();
        let mut next_row = Vec::new();
        for c in line.chars() {
            let tree = c.to_digit(10).unwrap() as i32;
            next_row.push(tree);
        }
        if !next_row.is_empty() {
            matrix.push(next_row);
        }
    }

    let d_matrix = DMatrix::from_fn(matrix.len(), matrix[0].len(), |r, c| matrix[r][c]);
    Ok(d_matrix)
}
