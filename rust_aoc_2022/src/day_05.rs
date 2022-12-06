use std::{path::Path, fs, error::Error};

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::{Parser, iterators::Pair};

#[derive(Parser)]
#[grammar = "./aoc_lib/day_05/day_05.pest"]
struct Day05Parser;

fn main() -> Result<(), Box<dyn Error>>{
    let path = Path::new("./res/day_05/day_05.csv");
    let content = fs::read_to_string(path)?;
    let parse = Day05Parser::parse(Rule::input, &content)?.next().unwrap();
    let mut towers = Towers::new();
    let mut moves: Vec<Move>= Vec::new();
    for inner in parse.into_inner() {
        match inner.as_rule() {
            Rule::towers => {
                towers = construct_towers(inner);
            },
            Rule::moves => {
                moves = construct_moves(inner);
            },
            Rule::EOI => (),
            _ => unreachable!()
        }
    }
    let mut towers_1 = towers.clone();
    let mut towers_2 = towers.clone();
    moves.iter().for_each(|m| m.execute_on(&mut towers_1));
    moves.iter().for_each(|m| m.execute_9001_on(&mut towers_2));
    println!("towers_1");
    towers_1.print_all_top();
    println!("towers_2");
    towers_2.print_all_top();
    Ok(())
}

#[derive(Debug)]
struct Move{
    n: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn new(n: usize, from: usize, to: usize) -> Self { Self { n, from, to } }

    pub fn execute_on(&self, towers: &mut Towers){
        for _ in 0..self.n {
            let cont = towers.pop(self.from);
            towers.put(cont, self.to);
        }
    }

    pub fn execute_9001_on(&self, towers: &mut Towers) {
        let mut to_move = Vec::new();
        for _ in 0..self.n{
            to_move.push(towers.pop(self.from));
        }
        to_move.into_iter().rev().for_each(|e| towers.put(e, self.to));
    }
}

#[derive(Debug, Clone)]
struct Towers {
    towers: Vec<Vec<String>>,
}

impl Towers {
    pub fn new() -> Self { Self { towers: Vec::new() } }
    pub fn put(&mut self, cont: String, tower: usize){
        if self.towers.len() < tower {
            for _ in 0..(tower - self.towers.len()) {
                self.towers.push(Vec::new());
            }
        }
        self.towers[tower -1].push(cont);
    }

    pub fn pop(&mut self, tower:usize) -> String {
        self.towers[tower-1].pop().expect("can't pop off an empty tower")
    }
    
    fn reverse_vertical(&mut self){
        self.towers.iter_mut().for_each(|t| t.reverse());
    }

    fn print_all_top(&self) {
        for tower in self.towers.iter() {
            print!("{:?} ", tower.last());
        }
        println!()
    }
}

fn construct_towers(inner: Pair<Rule>) -> Towers{
    let mut towers = Towers::new();

    for row in inner.into_inner() {
        match row.as_rule() {
            Rule::tower_row => {
                let mut tower = 0;
                for tower_entry in row.into_inner(){
                    tower += 1;
                    let entry = tower_entry.into_inner().next().unwrap();
                    match entry.as_rule() {
                        Rule::one_crate => {
                            let cont = entry.as_span().as_str().to_string();
                            towers.put(cont, tower);
                        },
                        Rule::empty => {
                        },
                        _ => panic!("not emtpy ore one_crate")
                    }
                }
            },
            Rule::tower_end_row => {

            }, 
            _ => panic!("not tower_row or tower_end_row")
        }
    }
    towers.reverse_vertical();
    println!("{:?}", towers);
    
    return towers;
}

fn construct_moves(inner: Pair<Rule>) -> Vec<Move>{
    let mut moves = Vec::new();

    for one_move in inner.into_inner() {
        match one_move.as_rule() {
            Rule::one_move => {
                let mut one_move = one_move.into_inner();
                let n = one_move.next().unwrap().as_str().parse().unwrap();
                let from = one_move.next().unwrap().as_str().parse().unwrap();
                let to = one_move.next().unwrap().as_str().parse().unwrap();
                moves.push(Move::new(n,from,to));
            },
            _ => panic!("one_move was no move")
        }
    }
    return moves;
}