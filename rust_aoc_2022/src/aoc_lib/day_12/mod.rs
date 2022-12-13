use std::{
    collections::VecDeque,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use csv::Position;
use rayon::result;

use super::utils::GenDynResult;

pub fn get_path_length_to_position_from_file(
    path: &Path,
) -> GenDynResult<Vec<Vec<Option<(usize, Direction)>>>> {
    let height_map = parse_height_map(path)?;
    let result = get_path_length_to_position(&height_map, height_map.e_position);

    Ok(result)
}

pub fn get_path_length_to_position(
    height_map: &HeightMap,
    (x, y): (usize, usize),
) -> Vec<Vec<Option<(usize, Direction)>>> {
    let mut path_length_map =
        vec![vec![None; height_map.height_map[0].len()]; height_map.height_map.len()];
    let mut queue = VecDeque::new();
    path_length_map[y][x] = Some((0, Direction::E));
    queue.push_back((x, y));

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        println!("look at: {:?}", (x,y));
        let neighbors_to_cur = get_all_neighbors_that_can_move_to_pos(height_map, (x, y));
        println!("with neighbors {:?}", neighbors_to_cur);
        neighbors_to_cur.iter().for_each(|(n_x, n_y)| {
            if path_length_map[*n_y][*n_x].is_none() {
                match path_length_map[y][x] {
                    Some((dest_lenghth, _)) => {
                        path_length_map[*n_y][*n_x] =
                            Some((dest_lenghth + 1, get_dir_from_to((*n_x, *n_y), (x, y))));
                        queue.push_back((*n_x, *n_y));
                    }
                    None => (),
                }
            }
        });
    }

    return path_length_map;
}

pub fn get_dir_from_to((f_x, f_y): (usize, usize), (d_x, d_y): (usize, usize)) -> Direction {
    if d_x == f_x && d_y + 1 == f_y {
        Direction::U
    } else if d_x == f_x + 1 && d_y == f_y {
        Direction::R
    } else if d_x == f_x && d_y == f_y + 1 {
        Direction::D
    } else if d_x + 1 == f_x && d_y == f_y {
        Direction::L
    } else if d_x == f_x && d_y == f_y {
        Direction::E
    } else {
        panic!("not direkt neighbors")
    }
}

pub fn get_all_neighbors_that_can_move_to_pos(
    height_map: &HeightMap,
    (x, y): (usize, usize),
) -> Vec<(usize, usize)> {
    let neighbors = get_all_direct_neighbors(height_map, (x, y));
    neighbors
        .into_iter()
        .filter(|(n_x, n_y)| height_map.height_map[y][x] <= height_map.height_map[*n_y][*n_x] + 1)
        .collect()
}

pub fn get_all_neighbors_pos_can_move_to(
    height_map: &HeightMap,
    (x, y): (usize, usize),
) -> Vec<(usize, usize)> {
    let neighbors = get_all_direct_neighbors(height_map, (x, y));
    neighbors
        .into_iter()
        .filter(|(n_x, n_y)| height_map.height_map[*n_y][*n_x] <= height_map.height_map[y][x] + 1)
        .collect()
}

pub fn get_all_direct_neighbors(
    height_map: &HeightMap,
    (x, y): (usize, usize),
) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    if x < height_map.height_map[0].len() - 1 {
        result.push((x + 1, y));
    }
    if x > 0 {
        result.push((x - 1, y));
    }
    if y > 0 {
        result.push((x, y - 1));
    }
    if y < height_map.height_map.len() - 1 {
        result.push((x, y + 1));
    }

    return result;
}

#[derive(Clone, Debug)]
pub enum Direction {
    U,
    D,
    L,
    R,
    E,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::U => write!(f, "^"),
            Direction::D => write!(f, "v"),
            Direction::L => write!(f, "<"),
            Direction::R => write!(f, ">"),
            Direction::E => write!(f, "O"),
        }
    }
}

pub fn parse_height_map(path: &Path) -> GenDynResult<HeightMap> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut height_map: Vec<Vec<usize>> = Vec::new();
    let mut s_position = (0, 0);
    let mut e_position = (0, 0);
    for (y, line) in reader.lines().enumerate() {
        let line = line?;
        let line = line.trim();
        if !line.is_empty() {
            let mut next_line_vec = Vec::new();
            for (x, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        s_position = (x, y);
                        next_line_vec.push('a' as usize - 'a' as usize);
                    }
                    'E' => {
                        e_position = (x, y);
                        next_line_vec.push('z' as usize - 'a' as usize);
                    }
                    c => {
                        next_line_vec.push(c as usize - 'a' as usize);
                    }
                }
            }
            height_map.push(next_line_vec);
        }
    }

    Ok(HeightMap {
        s_position,
        e_position,
        height_map,
    })
}

pub struct HeightMap {
    pub s_position: (usize, usize),
    pub e_position: (usize, usize),
    pub height_map: Vec<Vec<usize>>,
}
