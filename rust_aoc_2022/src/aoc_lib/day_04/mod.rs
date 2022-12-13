use std::{num::ParseIntError, path::Path, str::FromStr};

pub fn get_overlapping_count(path: &Path) -> Result<usize, String> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)
        .map_err(|e| e.to_string())?;

    let mut result = 0;

    for record in reader.records() {
        let record = record.map_err(|e| e.to_string())?;
        let elf_0: Intervall = record[0].parse()?;
        let elf_1: Intervall = record[1].parse()?;
        if elf_0.overlaps(&elf_1) {
            result += 1;
        }
    }
    return Ok(result);
}

pub fn get_containing_count(path: &Path) -> Result<usize, String> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)
        .map_err(|e| e.to_string())?;

    let mut result = 0;

    for record in reader.records() {
        let record = record.map_err(|e| e.to_string())?;
        let elf_0: Intervall = record[0].parse()?;
        let elf_1: Intervall = record[1].parse()?;
        if elf_0.contains(&elf_1) || elf_1.contains(&elf_0) {
            result += 1;
        }
    }
    return Ok(result);
}

#[derive(PartialEq, Eq, Debug)]
pub struct Intervall {
    left: usize,
    right: usize,
}

impl FromStr for Intervall {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<&str> = s.trim().split('-').collect();
        if numbers.len() != 2 {
            return Err(String::from("Wrong count of numbers"));
        }
        let left = numbers[0]
            .parse()
            .map_err(|err: ParseIntError| err.to_string())?;
        let right = numbers[1]
            .parse()
            .map_err(|err: ParseIntError| err.to_string())?;

        return Ok(Intervall::new(left, right));
    }
}

impl Intervall {
    pub fn new(left: usize, right: usize) -> Self {
        if right < left {
            return Self { left: 1, right: 0 };
        }
        Self { left, right }
    }

    pub fn contains(&self, other: &Self) -> bool {
        (self.left <= other.left && self.right >= other.right) || other.is_empty()
    }

    pub fn is_empty(&self) -> bool {
        self.left > self.right
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.left <= other.right && self.right >= other.left
            || self.right >= other.left && self.left <= other.right
    }
}

#[cfg(test)]
mod tests {
    use super::Intervall;

    #[test]
    fn parse_test() {
        assert_eq!(Intervall::new(1, 3), "1-3".parse().unwrap());
    }
}
