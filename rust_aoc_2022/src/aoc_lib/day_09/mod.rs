use std::{collections::HashSet, path::Path, fs::File, io::{BufReader, BufRead}};

pub fn make_moves_from_file(path: &Path, knots:usize) -> Result<HeadTailPositionsSeries, Box<dyn std::error::Error>> {
    let mut head_tail_pos = HeadTailPositionsSeries::new(knots);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines(){
        let line = line?;
        let line = line.trim();
        let line_split:Vec<_> = line.split(" ").collect();
        if line_split.len() == 2{
            for _ in 0..(line_split[1].parse::<usize>()?) {
                match line_split[0] {
                    "R" => head_tail_pos.move_head((1,0)),
                    "L" => head_tail_pos.move_head((-1,0)),
                    "U" => head_tail_pos.move_head((0,1)),
                    "D" => head_tail_pos.move_head((0,-1)),
                    x => panic!("unknowen direktion {}", x),
                }
            }
        }
    }

    return Ok(head_tail_pos);
}


pub struct HeadTailPositions{
    pub head_pos: (isize, isize),
    pub tail_pos: (isize, isize),
    pub head_pos_set: HashSet<(isize, isize)>,
    pub tail_pos_set: HashSet<(isize, isize)>
}

pub struct HeadTailPositionsSeries{
    pub head_tail_series: Vec<HeadTailPositions>
}

impl HeadTailPositionsSeries{
    pub fn new(knots: usize) -> Self{
        let mut series = Vec::new();
        for _ in 0..knots-1 {
            series.push(HeadTailPositions::new());
        }
        Self { head_tail_series: series }
    }

    pub fn move_head(&mut self, dir:(isize, isize)) {
        let mut dir = dir;
        for head_tails in self.head_tail_series.iter_mut() {
            let old_tail_pos = head_tails.tail_pos;
            head_tails.move_head(dir);
            dir.0 = head_tails.tail_pos.0 - old_tail_pos.0;
            dir.1 = head_tails.tail_pos.1 - old_tail_pos.1;
        }
    }
}

impl HeadTailPositions{
    pub fn new() -> Self {
        Self { head_pos: (0,0), tail_pos: (0,0), head_pos_set: HashSet::from([(0,0)]), tail_pos_set: HashSet::from([(0,0)]) }
    }

    pub fn move_head(&mut self, dir: (isize, isize)){
        self.head_pos.0 += dir.0;
        self.head_pos.1 += dir.1;
        self.head_pos_set.insert(self.head_pos);
        if isize::abs(self.head_pos.0 - self.tail_pos.0) > 1{
            self.tail_pos.0 += isize::signum(self.head_pos.0 - self.tail_pos.0);
            if isize::abs(self.head_pos.1 - self.tail_pos.1) > 0{
                self.tail_pos.1 += isize::signum(self.head_pos.1 - self.tail_pos.1);
            }
        }
        if isize::abs(self.head_pos.1 - self.tail_pos.1) > 1{
            self.tail_pos.1 += isize::signum(self.head_pos.1 - self.tail_pos.1);
            if isize::abs(self.head_pos.0 - self.tail_pos.0) > 0{
                self.tail_pos.0 += isize::signum(self.head_pos.0 - self.tail_pos.0);
            }
        }
        self.tail_pos_set.insert(self.tail_pos);
    }
}