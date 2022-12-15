use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, self, Write},
    path::Path, hash::Hash,
};

use super::utils::GenDynResult;

pub struct Cave {
    pub fields: HashMap<(usize, usize), CaveFill>,
}

pub struct CavePartTwo {
    pub fields: HashMap<usize, HashMap<usize, CaveFill>>
}

impl CavePartTwo {
    pub fn new() -> Self { Self { fields:HashMap::new() } }

    pub fn from_cave(cave:Cave) -> Self {
        let mut new_cave = CavePartTwo::new();
        for ((x,y), fill) in cave.fields {
            if !new_cave.fields.contains_key(&x) {
                new_cave.fields.insert(x, HashMap::new());
            }
            new_cave.fields.get_mut(&x).unwrap().insert(y, fill);
        }

        new_cave
    }


    pub fn throw_sand_at_part_two(&mut self, throw_pos: (usize, usize), floor_y:usize) -> Option<(usize, usize)> {
        let landing = self.get_landing_pos_part_two(throw_pos, floor_y)?;
        let coloumn = self.fields.get_mut(&landing.0).unwrap();
        coloumn.insert(landing.1, CaveFill::Sand);
        return Some(landing);
    }

    pub fn get_landing_pos_part_two(&self, (x_s, y_s): (usize, usize), floor_y:usize) -> Option<(usize, usize)> {
        let default = HashMap::new();
        let binding = [floor_y];
        let mut landing = self
            .fields
            .get(&x_s)
            .unwrap_or(&default)
            .keys()
            .chain(binding.iter())
            .filter(|y| **y >= y_s)
            .min()?;
        let (l_x, l_y) = (x_s, landing - 1);
        let try_left = (l_x - 1, l_y + 1);
        let try_right = (l_x + 1, l_y + 1);

        if !self.fields.contains_key(&try_left.0) && !self.fields.get(&try_left.0).unwrap().contains_key(&try_left.1) && try_left.1 != floor_y{
            return self.get_landing_pos_part_two(try_left, floor_y);
        } 
        if !self.fields.contains_key(&try_right.0) && !self.fields.get(&try_right.0).unwrap().contains_key(&try_right.1) && try_right.1 != floor_y {
            return self.get_landing_pos_part_two(try_right, floor_y);
        }
        return Some((x_s, l_y));
    }
    

    pub fn throw_sand_at_part_one(&mut self, throw_pos: (usize, usize)) -> Option<(usize, usize)> {
        let landing = self.get_landing_pos_part_one(throw_pos)?;
        let coloumn = self.fields.get_mut(&landing.0).unwrap();
        coloumn.insert(landing.1, CaveFill::Sand);
        return Some(landing);
    }

    pub fn get_landing_pos_part_one(&self, (x_s, y_s): (usize, usize)) -> Option<(usize, usize)> {
        let default = HashMap::new();
        let mut landing = self
            .fields
            .get(&x_s)
            .unwrap_or(&default)
            .keys()
            .filter(|y| **y >= y_s)
            .min()?;
        println!("landing {landing}");
        let (l_x, l_y) = (x_s, landing - 1);
        println!("x_s {x_s}, l_x {l_x}, l_y {l_y}, landing {landing}");
        let try_left = (l_x - 1, l_y + 1);
        let try_right = (l_x + 1, l_y + 1);

        if !self.fields.contains_key(&try_left.0) && !self.fields.get(&try_left.0).unwrap().contains_key(&try_left.1){
            return self.get_landing_pos_part_one(try_left);
        } 
        if !self.fields.contains_key(&try_right.0) && !self.fields.get(&try_right.0).unwrap().contains_key(&try_right.1) {
            return self.get_landing_pos_part_one(try_right);
        }
        return Some((x_s, l_y));
    }

    pub fn print_cave(&self) -> Option<()> {
        let min_x = self.fields.keys().min()?;
        let max_x = self.fields.keys().max()?;
        let min_y = self.fields.values().flat_map(|x| x.keys()).min()?;
        let max_y = self.fields.values().flat_map(|x| x.keys()).max()?;

        for y in *min_y..=*max_y {
            for x in *min_x..=*max_x {
                if let Some(coloumn) = self.fields.get(&x) {
                    if let Some(fill) = coloumn.get(&y) {
                        match fill {
                            CaveFill::Stone => print!("#"),
                            CaveFill::Sand => print!("o"),
                        }
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

impl Cave {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }


    pub fn throw_sand_at(&mut self, throw_pos: (usize, usize)) -> Option<(usize, usize)> {
        let landing = self.get_landing_pos(throw_pos)?;
        self.fields.insert(landing, CaveFill::Sand);
        return Some(landing);
    }

    pub fn get_landing_pos(&self, (x_s, y_s): (usize, usize)) -> Option<(usize, usize)> {
        let mut landing = self
            .fields
            .keys()
            .filter(|(x, y)| *x == x_s && *y >= y_s)
            .min_by(|(max_x, max_y), (x, y)| max_y.cmp(y))?
            .clone();
        landing.1 -= 1;
        let (l_x, l_y) = landing;
        let try_left = (l_x - 1, l_y + 1);
        let try_right = (l_x + 1, l_y + 1);

        if !self.fields.contains_key(&try_left) {
            return self.get_landing_pos(try_left);
        } 
        if !self.fields.contains_key(&try_right) {
            return self.get_landing_pos((l_x + 1, l_y + 1));
        }
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
        io::stdout().flush().unwrap();

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
            .map(|x| (x[0], x[1]))
            .collect::<Vec<_>>();
        for i in 0..coordinates.len() - 1 {
            cave.fill_from_to(coordinates[i], coordinates[i + 1]);
        }
    }

    Ok(cave)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
	let path = Path::new(r"./res/day_14/day_14.csv");

	let cave = parse_stone_structures(path)?;
	let mut cave = CavePartTwo::from_cave(cave);
	cave.print_cave();
	let mut throw_count = 0;
	
	while let Some(point) = cave.throw_sand_at_part_one((500,0)) {
		throw_count += 1;
	}

	cave.print_cave();
	println!("throw count: {throw_count}");

	let mut cave = parse_stone_structures(path)?;
	let floor_y = cave.fields.keys().map(|x| x.1).max().map(|x|x+2).unwrap();
	let mut cave = CavePartTwo::from_cave(cave);
	let mut throw_count = 0;
	loop {
		let landing_point = cave.throw_sand_at_part_two((500,0), floor_y).unwrap();
		throw_count += 1;
		if landing_point == (500,0) {
			break;
		}
	}


	println!("throw count part two: {throw_count}");

	Ok(())
}