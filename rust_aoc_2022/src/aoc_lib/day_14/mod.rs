use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use super::utils::GenDynResult;

pub struct Cave {
    fields: HashMap<(usize, usize), CaveFill>,
}

impl Cave {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    pub fn throw_sand_at(&mut self, (x_s, y_s): (usize, usize)) -> Option<(usize, usize)> {
        let mut landing @ (l_x, l_y) = self
            .fields
            .keys()
            .filter(|(x, y)| *x == x_s && *y >= y_s)
            .max_by(|(max_x, max_y), (x, y)| max_y.cmp(y))?
            .clone();
        if !self.fields.contains_key(&(l_x - 1, l_y + 1)) {
            landing = self.throw_sand_at((l_x - 1, l_y + 1))?;
        } else if !self.fields.contains_key(&(l_x + 1, l_y + 1)) {
            landing = self.throw_sand_at((l_x + 1, l_y + 1))?;
        }
        self.fields.insert(landing, CaveFill::Sand);
        return Some(landing);
    }

    pub fn fill_from_to(&mut self, (f_x, f_y): (usize, usize), (d_x, d_y): (usize, usize)) {
        if f_y == d_y {
            let min = usize::min(f_x, d_x);
            let max = usize::max(f_x, d_x);
            for i in min..=max {
                self.fields.insert((i, d_y), CaveFill::Stone);
            }
        } else if f_x == d_x {
            let min = usize::min(f_y, d_y);
            let max = usize::max(f_y, d_y);
            for i in min..=max {
                self.fields.insert((d_x, i), CaveFill::Stone);
            }
        }
    }

    pub fn print_cave(&self) -> Option<()> {
        let min_x = self.fields.keys().map(|f| f.0).min()?;
        let max_x = self.fields.keys().map(|f| f.0).max()?;
        let min_y = self.fields.keys().map(|f| f.1).min()?;
        let max_y = self.fields.keys().map(|f| f.1).max()?;

        let row_count = max_y - min_y;
        let column_count = max_x - min_x;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if let Some(fill) = self.fields.get(&(x, y)) {
                    match fill {
                        CaveFill::Stone => print!("#"),
                        CaveFill::Sand => print!("o"),
                    }
                } else {
                    print!(".")
                }
            }
            println!("")
        }

        Some(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CaveFill {
    Stone,
    Sand,
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
            .map(|x| (x[0],x[1]))
            .collect::<Vec<_>>();
        for i in 0..coordinates.len()-1{
            cave.fill_from_to(coordinates[i], coordinates[i+1]);
        }
    }

    Ok(cave)
}
