extern crate pest;
use std::{path::Path, fs};

use pest::{Parser, iterators::Pair};
use rayon::result;

use super::utils::GenDynResult;


#[derive(Parser)]
#[grammar = r"./aoc_lib/day_11/day_11.pest"]
pub struct MonkeyParser;


pub fn parse_monkeys(path:&Path) -> GenDynResult<Vec<Monkey>>{
    let mut monkeys = Vec::new();
    let file_cont = fs::read_to_string(path)?;
    let parse = MonkeyParser::parse(Rule::input, &file_cont)?.next().unwrap();
    for monkey in parse.into_inner() {
        match monkey.as_rule() {
            Rule::monkey => {
                monkeys.push(parse_monkey(monkey)?);
            },
            Rule::EOI => {

            },
            _ => unreachable!()
        }
    }

    Ok(monkeys)
}

pub fn parse_monkey(monkey: Pair<Rule>) ->GenDynResult<Monkey> {
    let mut monkey = monkey.into_inner();
    let monkey_num = monkey.next().unwrap().as_str().parse::<usize>();
    let starting_items = parse_starting_items(monkey.next().unwrap())?;
    let operation = parse_operation(monkey.next().unwrap())?;
    let test = parse_test(monkey.next().unwrap())?;

    Ok(Monkey::new(starting_items, operation, test))
}

pub fn parse_starting_items(starting_items: Pair<Rule>) -> GenDynResult<Vec<usize>>{
    let mut result = Vec::new();
    match starting_items.as_rule() {
        Rule::starting_items => {
            for int in starting_items.into_inner() {
                result.push(int.as_str().parse()?);
            }
        },
        _ => panic!("tried to parse {} as starting_item", starting_items.as_str())
    }
    Ok(result)
}

pub fn parse_operation(operation: Pair<Rule>) -> GenDynResult<Box<dyn Fn(usize)->usize>>{
    let expression = operation.into_inner().next().unwrap();
    match expression.as_rule() {
        Rule::addition => {
            let mut expression = expression.into_inner();
            let lhs = expression.next().unwrap();
            let rhs = expression.next().unwrap();
            let rule_tup@(lhs_rule, rhs_rule) = (lhs.as_rule(), rhs.as_rule());
            match rule_tup {
                (Rule::int, Rule::int) => {
                    let lhs = lhs.as_str().parse::<usize>()?;
                    let rhs = rhs.as_str().parse::<usize>()?;
                    return Ok(Box::new(move |old| lhs + rhs));
                },
                (Rule::old, Rule::int) => {
                    let rhs = rhs.as_str().parse::<usize>()?;
                    return Ok(Box::new(move |old| old + rhs));
                },
                (Rule::int, Rule::old) => {
                    let lhs = lhs.as_str().parse::<usize>()?;
                    return Ok(Box::new(move |old| lhs + old));
                },
                (Rule::old, Rule::old) => {
                    return Ok(Box::new(move |old| old + old));
                },
                _=> unreachable!()
            }

        },
        Rule::multiplikation => {
            let mut expression = expression.into_inner();
            let lhs = expression.next().unwrap();
            let rhs = expression.next().unwrap();
            let rule_tup@(lhs_rule, rhs_rule) = (lhs.as_rule(), rhs.as_rule());
            match rule_tup {
                (Rule::int, Rule::int) => {
                    let lhs = lhs.as_str().parse::<usize>()?;
                    let rhs = rhs.as_str().parse::<usize>()?;
                    return Ok(Box::new(move |old| lhs * rhs));
                },
                (Rule::old, Rule::int) => {
                    let rhs = rhs.as_str().parse::<usize>()?;
                    return Ok(Box::new(move |old| old * rhs));
                },
                (Rule::int, Rule::old) => {
                    let lhs = lhs.as_str().parse::<usize>()?;
                    return Ok(Box::new(move |old| lhs * old));
                },
                (Rule::old, Rule::old) => {
                    return Ok(Box::new(move |old| old * old));
                },
                _=> unreachable!()
            }
        },
        _ => unreachable!()
    }
}

pub fn parse_test(test: Pair<Rule>) -> GenDynResult<Test> {

    let mut test = test.into_inner();
    let divisor = test.next().unwrap().as_str().parse()?;
    let true_monkey = test.next().unwrap().as_str().parse()?;
    let false_monkey = test.next().unwrap().as_str().parse()?;
    
    Ok(Test::new(divisor, true_monkey, false_monkey))
}

pub struct Monkey {
    pub starting_items: Vec<usize>,
    pub operation: Box<dyn Fn(usize)->usize>,
    pub test: Test,
    pub inspection_count: usize,
}

impl Monkey {
    pub fn new(starting_items: Vec<usize>, operation: Box<dyn Fn(usize)->usize>, test: Test) -> Self { Self { starting_items, operation, test, inspection_count:0 } }

    pub fn take_turn(&mut self) -> Vec<(usize, usize)>{
        let mut result = Vec::new();
        let items = std::mem::replace(&mut self.starting_items, Vec::new());
        for item in items {
            let item_new = (self.operation)(item) / 3;
            if item_new % self.test.divisor == 0 {
                result.push((item_new, self.test.true_monkey));
            } else {
                result.push((item_new, self.test.false_monkey));
            }
            self.inspection_count += 1;
        }

        return result;
    }

    pub fn catch_item(&mut self, item: usize){
        self.starting_items.push(item);
    }
}

#[derive(Debug)]
pub struct Test {
    pub divisor: usize,
    pub true_monkey: usize,
    pub false_monkey: usize,
}

impl Test {
    pub fn new(divisor: usize, true_monkey: usize, false_monkey: usize) -> Self { Self { divisor, true_monkey, false_monkey } }
}