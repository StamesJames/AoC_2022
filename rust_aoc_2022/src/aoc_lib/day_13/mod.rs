extern crate pest;

use std::{fs, path::Path};

use pest::{iterators::Pair, Parser};

use super::utils::GenDynResult;

#[derive(Parser)]
#[grammar = r"./aoc_lib/day_13/day_13.pest"]
struct PacketsParser;

#[derive(PartialEq, Eq, Debug)]
pub enum PacketData {
    Int(usize),
    List(Vec<PacketData>),
}

impl PacketData {
    pub fn push(&mut self, packet: PacketData) {
        match self {
            PacketData::Int(_) => (),
            PacketData::List(l) => l.push(packet),
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            PacketData::Int(i_l) => match other {
                PacketData::Int(i_r) => i_l.cmp(i_r),
                PacketData::List(_) => listify(*i_l).cmp(other),
            },
            PacketData::List(l_l) => match other {
                PacketData::Int(i_r) => self.cmp(&listify(*i_r)),
                PacketData::List(l_r) => {
                    for (l_e, r_e) in l_l.iter().zip(l_r) {
                        match l_e.cmp(r_e) {
                            std::cmp::Ordering::Equal => (),
                            c => return c,
                        }
                    }
                    return l_l.len().cmp(&l_r.len());
                }
            },
        }
    }
}

pub fn listify(i: usize) -> PacketData {
    PacketData::List(vec![PacketData::Int(i)])
}

pub fn parse_package_data(path: &Path) -> GenDynResult<Vec<(PacketData, PacketData)>> {
    let mut pairs_vec = Vec::new();
    let cont = fs::read_to_string(path)?;
    let parse = PacketsParser::parse(Rule::input, &cont)?.next().unwrap();
    for pair in parse.into_inner() {
        match pair.as_rule() {
            Rule::pair => {
                let mut pair = pair.into_inner();
                let first = parse_list(pair.next().unwrap())?;
                let second = parse_list(pair.next().unwrap())?;
                pairs_vec.push((first, second));
            }
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }

    Ok(pairs_vec)
}

pub fn parse_list(list: Pair<Rule>) -> GenDynResult<PacketData> {
    let mut packet = PacketData::List(Vec::new());
    for entry in list.into_inner() {
        let entry = entry.into_inner().next().unwrap();
        match entry.as_rule() {
            Rule::int => packet.push(PacketData::Int(entry.as_str().parse()?)),
            Rule::list => packet.push(parse_list(entry)?),
            _ => unreachable!(),
        }
    }

    Ok(packet)
}
