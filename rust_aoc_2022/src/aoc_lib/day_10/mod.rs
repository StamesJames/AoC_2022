use std::{path::Path, fs::File, io::{BufReader, BufRead}};

pub fn get_img_from_file(path:&Path) -> Result<Vec<bool>, Box<dyn std::error::Error>> {
    let mut img = Vec::new();
    let register_list = get_register_list_from_file(path)?;


    for (cycle, register) in register_list.iter().enumerate() {
        if isize::abs((cycle % 40) as isize - register) <= 1 {
            img.push(true);
        }else {
            img.push(false);
        }
    }

    Ok(img)
}

pub fn get_register_list_from_file(path:&Path) -> Result<Vec<isize>, Box<dyn std::error::Error>> {
    let mut register = 1;
    let mut register_list = Vec::new();
    register_list.push(register);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines(){
        let line = line?;
        let line = line.trim();
        let line_split = line.split(" ").collect::<Vec<_>>();
        match &line_split[0] {
            &"noop" => register_list.push(register),
            &"addx" => {
                register_list.push(register);
                register += line_split[1].parse::<isize>()?;
                register_list.push(register);
            },
            _ => {}
        }
    }


    Ok(register_list)
}