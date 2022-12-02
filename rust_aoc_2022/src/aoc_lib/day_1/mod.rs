use std::{path::Path, fs::File, io::{BufReader, BufRead}};

pub fn find_most_cal_elf(path: &Path) -> (usize, usize) {
    let mut max_calories = 0;
    let mut max_elf_index = 0;
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut current_calorie_count = 0;
    let mut current_elf_index = 0;
    for line in reader.lines().map(|x| x.unwrap()) {
        if line.trim().is_empty() {
            if current_calorie_count > max_calories {
                max_calories = current_calorie_count;
                max_elf_index = current_elf_index;
            }
            current_calorie_count = 0;
            current_elf_index += 1;
        } else {
            current_calorie_count += line.parse::<usize>().unwrap();
        }
    }

    if current_calorie_count > max_calories {
        max_calories = current_calorie_count;
        max_elf_index = current_elf_index;
    }

    return (max_calories, max_elf_index);
}

pub fn find_3_most_cal_elfs(path: &Path) -> [usize; 3] {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut max_3_elfes = [0,0,0];
    let mut current_calorie_count = 0;

    for line in reader.lines().map(|x| x.unwrap()) {
        if line.trim().is_empty() {
            add_to_3_max(current_calorie_count, &mut max_3_elfes);
            current_calorie_count = 0;
        } else {
            current_calorie_count += line.parse::<usize>().unwrap();
        }
    }

    return max_3_elfes;
}

fn add_to_3_max(x:usize, values: &mut [usize; 3]){
    let mut x = x;
    for v in values {
        if x > *v {
            let tmp = *v;
            *v = x;
            x = tmp;
        }
    }
}
