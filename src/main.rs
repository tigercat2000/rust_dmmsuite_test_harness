use pest::Parser;
#[macro_use]
extern crate pest;
use rust_dmmsuite::{DMMParser, Rule, DMM};

fn main() {
    let map = include_str!("../MetaStation.dmm");

    let parse = DMMParser::parse(Rule::map, map)
        .expect("Failed parse")
        .next()
        .unwrap();
    let map = DMM::DMM::from_parser(parse);
    println!("Map: {:#?}", map);
}
