use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use pest::pratt_parser::Op;

use super::utils::GenDynResult;

pub struct Cave {
    positive_cols: Vec<Vec<CaveFill>>,
    negative_cols: Vec<Vec<CaveFill>>,
}

impl Cave {
    pub fn throw_sand_from_with_floor(
        &mut self,
        throw_point: (usize, usize),
        floor_y: usize,
    ) -> Option<(usize, usize)> {
        let landing = self.get_landing_pos_with_floor(throw_point, floor_y)?;
        self.positive_cols[landing.0][landing.1] = CaveFill::Sand;
        Some(landing)
    }

    pub fn get_landing_pos_with_floor(
        &mut self,
        throw_point: (usize, usize),
        floor_y: usize,
    ) -> Option<(usize, usize)> {
        let col = &self.positive_cols[throw_point.0];
        let hit = col
            .iter()
            .enumerate()
            .find(|x| x.0 >= throw_point.1 && (*x.1 == CaveFill::Stone || *x.1 == CaveFill::Sand))?
            .0;
        let left_col = &mut self.positive_cols[throw_point.0 - 1];
        if left_col.len() > hit && left_col[hit] == CaveFill::Air {
            return self.get_landing_pos_with_floor((throw_point.0 - 1, hit), floor_y);
        } else if left_col.len() <= hit {
            for _ in left_col.len()..=floor_y {
                left_col.push(CaveFill::Air);
            }
            left_col[floor_y] = CaveFill::Stone;
            return Some((throw_point.0 - 1, floor_y - 1));
        }
        if self.positive_cols.len() <= throw_point.0 + 1{
            for _ in self.positive_cols.len()..=throw_point.0 + 1 {
                self.positive_cols.push(Vec::new());
            }
        }
        let right_col = &mut self.positive_cols[throw_point.0 + 1];

        if right_col.len() > hit && right_col[hit] == CaveFill::Air {
            return self.get_landing_pos_with_floor((throw_point.0 + 1, hit), floor_y);
        } else if right_col.len() <= hit {
            for _ in right_col.len()..=floor_y {
                right_col.push(CaveFill::Air);
            }
            right_col[floor_y] = CaveFill::Stone;
            return Some((throw_point.0 + 1, floor_y - 1));
        }

        Some((throw_point.0, hit - 1))
    }

    pub fn throw_sand_from(&mut self, throw_point: (usize, usize)) -> Option<(usize, usize)> {
        let landing = self.get_landing_pos(throw_point)?;
        self.positive_cols[landing.0][landing.1] = CaveFill::Sand;
        Some(landing)
    }

    pub fn get_landing_pos(&mut self, throw_point: (usize, usize)) -> Option<(usize, usize)> {
        let col = &self.positive_cols[throw_point.0];
        let hit = col
            .iter()
            .enumerate()
            .find(|x| x.0 >= throw_point.1 && (*x.1 == CaveFill::Stone || *x.1 == CaveFill::Sand))?
            .0;
        let left_col = &self.positive_cols[throw_point.0 - 1];
        if left_col.len() > hit && left_col[hit] == CaveFill::Air {
            return self.get_landing_pos((throw_point.0 - 1, hit));
        } else if left_col.len() <= hit {
            return None;
        }
        if self.positive_cols.len() <= throw_point.0 + 1{
            for _ in self.positive_cols.len()..=throw_point.0 + 1 {
                self.positive_cols.push(Vec::new());
            }
        }
        let right_col = &self.positive_cols[throw_point.0 + 1];
        if right_col.len() > hit && right_col[hit] == CaveFill::Air {
            return self.get_landing_pos((throw_point.0 + 1, hit));
        } else if right_col.len() <= hit {
            return None;
        }

        Some((throw_point.0, hit - 1))
    }

    pub fn new() -> Self {
        Self {
            positive_cols: Vec::new(),
            negative_cols: Vec::new(),
        }
    }

    pub fn fill_from_to(&mut self, from: (usize, usize), to: (usize, usize)) {
        let min_y = usize::min(from.1, to.1);
        let max_y = usize::max(from.1, to.1);
        let min_x = usize::min(from.0, to.0);
        let max_x = usize::max(from.0, to.0);

        if self.positive_cols.len() <= max_x {
            for _ in self.positive_cols.len()..=max_x {
                self.positive_cols.push(Vec::new());
            }
        }

        if from.0 == to.0 {
            let col = &mut self.positive_cols[from.0];
            if col.len() <= max_y {
                for _ in col.len()..=max_y {
                    col.push(CaveFill::Air);
                }
            }
            for i in min_y..=max_y {
                col[i] = CaveFill::Stone;
            }
        } else if from.1 == to.1 {
            for i in min_x..=max_x {
                let col = &mut self.positive_cols[i];
                if col.len() <= from.1 {
                    for _ in 0..=(from.1 - col.len()) {
                        col.push(CaveFill::Air);
                    }
                }
                col[from.1] = CaveFill::Stone;
            }
        }
    }

    pub fn print_cave(&self) {
        let max_y = self.positive_cols.iter().map(|c| c.len()).max().unwrap();
        let min_x = self
            .positive_cols
            .iter()
            .enumerate()
            .filter(|(x, col)| col.contains(&CaveFill::Stone) || col.contains(&CaveFill::Sand))
            .map(|x| x.0)
            .min()
            .unwrap();
        for y in 0..max_y {
            for x in min_x..self.positive_cols.len() {
                let col = &self.positive_cols[x];
                if y < col.len() {
                    let fill = col[y];
                    match fill {
                        CaveFill::Air => print!("."),
                        CaveFill::Sand => print!("o"),
                        CaveFill::Stone => print!("#"),
                    }
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    pub fn get_max_y(&self) -> usize {
        self.positive_cols
            .iter()
            .flat_map(|col| col.iter().enumerate().filter(|x| *x.1 == CaveFill::Stone))
            .map(|x| x.0)
            .max().unwrap()
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CaveFill {
    #[default]
    Air,
    Sand,
    Stone,
}

pub fn parse_stone_structures(path: &Path) -> GenDynResult<Cave> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut cave = Cave::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        let coordinates = line
            .split("->")
            .map(|x| x.trim().split(","))
            .map(|x| x.map(|x| x.parse::<usize>().unwrap()))
            .map(|x| x.collect::<Vec<_>>())
            .map(|x| (x[0], x[1]))
            .collect::<Vec<_>>();
        for i in 0..coordinates.len() - 1 {
            cave.fill_from_to(coordinates[i], coordinates[i + 1]);
        }
    }

    Ok(cave)
}
