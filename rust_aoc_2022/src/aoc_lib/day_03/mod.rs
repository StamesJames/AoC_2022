use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
};

use rayon::prelude::*;

pub fn get_badge_priority_sum_par(path: &Path) -> usize {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let result = AtomicUsize::new(0);
    let line_chunks: Vec<[String; 3]> = reader
        .lines()
        .map(Result::unwrap)
        .array_chunks::<3>()
        .collect();

    line_chunks.par_iter().for_each(|[line_1, line_2, line_3]| {
        for c in line_1.chars() {
            if line_2.contains(c) && line_3.contains(c) {
                result.fetch_add(char_to_priority(&c), Ordering::SeqCst);
                break;
            }
        }
    });

    return result.load(Ordering::SeqCst);
}

pub fn get_badge_priority_sum(path: &Path) -> usize {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut result = 0;
    let line_chunks = reader.lines().map(Result::unwrap).array_chunks::<3>();

    for [line_1, line_2, line_3] in line_chunks {
        for c in line_1.chars() {
            if line_2.contains(c) && line_3.contains(c) {
                result += char_to_priority(&c);
                break;
            }
        }
    }

    return result;
}
pub fn get_badge_priority_sum_hashset(path: &Path) -> usize {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut result = 0;
    let line_chunks = reader.lines().map(Result::unwrap).array_chunks::<3>();

    for [line_1, line_2, line_3] in line_chunks {
        let line_2: HashSet<char> = HashSet::from_iter(line_2.chars());
        let line_3: HashSet<char> = HashSet::from_iter(line_3.chars());
        for c in line_1.chars() {
            if line_2.contains(&c) && line_3.contains(&c) {
                result += char_to_priority(&c);
                break;
            }
        }
    }

    return result;
}

pub fn get_priority_sum(path: &Path) -> usize {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut result = 0;

    for line in reader.lines().map(Result::unwrap) {
        let line = line.trim();
        let (first, second) = line.split_at(line.len() / 2);
        for c in first.chars() {
            if second.contains(c) {
                result += char_to_priority(&c);
                break;
            }
        }
    }

    return result;
}

pub fn char_to_priority(c: &char) -> usize {
    if c.is_ascii_uppercase() {
        (*c as usize) - 64 + 26
    } else if c.is_ascii_lowercase() {
        (*c as usize) - 96
    } else {
        panic!("not a ASCII symbol")
    }
}
