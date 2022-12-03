use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
};

pub fn get_rps_score_with_endings(path: &Path) -> usize {
    let mut result = 0;
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines().map(|x| x.unwrap()).filter(|s| !s.is_empty()) {
        let line = line.split_whitespace().collect::<Vec<&str>>();
        let oponent: RPS = line[0].parse::<RPS>().unwrap();
        let ending:Ending = line[1].parse::<Ending>().unwrap();
        let my_rps = ending.get_rps_for_ending(&oponent);
        result += ending.get_score();
        result += my_rps.get_score();
    }

    return result;
}

pub fn get_rps_score(path: &Path) -> usize {
    let mut result = 0;

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines().map(|x| x.unwrap()) {
        let line = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<RPS>>();
        result += line[1].get_score() + line[1].get_score_against(&line[0]);
    }

    return result;
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum RPS {
    Rock,
    Paper,
    Sciss,
}

impl RPS {
    pub fn get_score(&self) -> usize {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Sciss => 3,
        }
    }

    pub fn get_ending(&self, other:&RPS) -> Ending {
        match self {
            RPS::Rock => match other {
                RPS::Rock => Ending::Draw,
                RPS::Paper => Ending::Lose,
                RPS::Sciss => Ending::Win,
            },
            RPS::Paper => match other {
                RPS::Rock => Ending::Win,
                RPS::Paper => Ending::Draw,
                RPS::Sciss => Ending::Lose,
            },
            RPS::Sciss => match other {
                RPS::Rock => Ending::Lose,
                RPS::Paper => Ending::Win,
                RPS::Sciss => Ending::Draw,
            },
        }
    }

    pub fn get_score_against(&self, other: &RPS) -> usize {
        self.get_ending(other).get_score()
    }

    pub fn get_win(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Sciss,
            RPS::Sciss => RPS::Rock,
        }
    }

    pub fn get_lose(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Sciss,
            RPS::Paper => RPS::Rock,
            RPS::Sciss => RPS::Paper,
        }
    }

    pub fn get_draw(&self) -> RPS {
        self.clone()
    }
}

impl FromStr for RPS {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(RPS::Rock),
            "B" | "Y" => Ok(RPS::Paper),
            "C" | "Z" => Ok(RPS::Sciss),
            x => Err(format!("{x} not the right symbol")),
        }
    }
}

#[derive(Debug)]
pub enum Ending {
    Win,
    Lose,
    Draw,
}

impl Ending {
    pub fn get_score(&self) -> usize {
        match self {
            Ending::Win => 6,
            Ending::Lose => 0,
            Ending::Draw => 3,
        }
    }

    pub fn get_rps_for_ending(&self, other: &RPS) -> RPS {
        match self {
            Ending::Win => other.get_win(),
            Ending::Lose => other.get_lose(),
            Ending::Draw => other.get_draw(),
        }
    }
}

impl FromStr for Ending {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Ending::Lose),
            "Y" => Ok(Ending::Draw),
            "Z" => Ok(Ending::Win),
            x => Err(format!("{x} not the right symbol")),
        }
    }
}
