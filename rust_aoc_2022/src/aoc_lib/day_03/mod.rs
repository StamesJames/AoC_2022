use std::{fs::File, io::{BufReader, BufRead}, path::Path, rc::Rc, collections::HashSet};


pub fn get_badge_priority_sum(path: &Path) -> usize {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut result = 0;
    let mut line_chunks = reader.lines().map(Result::unwrap).array_chunks::<3>();
    
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
