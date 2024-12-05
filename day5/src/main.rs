#[path = "./utils/file.rs"]
mod file;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE", required = true)]
    file: String,
}

struct OrderingPair {
    first: i32,
    second: i32,
}

fn main() {
    let cli = Cli::parse();
    let file_path = cli.file;
    let raw_data = file::load_file(&file_path);
    let blocks = get_blocks_from_raw_data(&raw_data);
    let ordering_rules: Vec<&'a str> = get_ordering_rules_from_ordering_rule_pairs(&blocks[0]);
    let correctly_ordered_updates = get_correctly_ordered_updates(&blocks[1], &ordering_rules);
}

fn get_blocks_from_raw_data(data: &String) -> Vec<Vec<&str>> {
    let mut lines = data.lines().peekable();
    let mut blocks = Vec::new();
    while lines.peek().is_some() {
        let block: Vec<_> = lines.by_ref().take_while(|l| l.trim().len() > 0).collect();
        blocks.push(block);
    }
    return blocks;
}

fn get_ordering_rules_from_ordering_rule_pairs(ordering_rule_pairs: &Vec<&str>) -> Vec<Vec<i32>> {
    let mut ordering_rules = Vec::new();
    for ordering_pair_str in ordering_rule_pairs {
        let ordering_pair: Vec<i32> = ordering_pair_str
            .split('|')
            .map(|op| op.parse::<i32>().unwrap())
            .collect();
        ordering_rules.push(ordering_pair);
    }
    return ordering_rules;
}

fn get_correctly_ordered_updates(
    update_strings: &Vec<&'a str>,
    ordering_rules: &Vec<Vec<i32>>,
) -> Vec<&str> {
    let mut correctly_ordered_updates: Vec<&str> = Vec::new();
    for update_string in update_strings {
        let numbers: Vec<i32> = update_string
            .split(',')
            .map(|us| us.parse::<i32>().unwrap())
            .collect();
        for i in 0..numbers.len() {}
    }
    return correctly_ordered_updates;
}
