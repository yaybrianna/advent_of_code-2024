#[path = "./utils/file.rs"]
mod file;


use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE", required = true)]
    file: String,
}

fn main() {
    let cli = Cli::parse();
    let file_path = cli.file;
    let raw_data = file::load_file(&file_path);
    let blocks = get_blocks_from_raw_data(&raw_data);
    let ordering_rules: Vec<Vec<i32>> = get_ordering_rules_from_ordering_rule_pairs(&blocks[0]);
    let updates = get_updates_as_i32_vec(&blocks[1]);
    let valid_updates =
        get_indexes_of_correctly_sorted_updates(&updates, &ordering_rules);
    let middle_pages = get_middle_page_numbers_from_valid_updates(&valid_updates);
    let mut sum = 0;
    for page in middle_pages {
        sum += page;
    }
    println!("Middle Pages Sum: {}", sum);
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

fn get_updates_as_i32_vec(update_strings: &Vec<&str>) -> Vec<Vec<i32>> {
    let mut updates: Vec<Vec<i32>> = Vec::new();
    for update_string in update_strings {
        let numbers: Vec<i32> = update_string
            .split(',')
            .map(|us| us.parse::<i32>().unwrap())
            .collect();

        updates.push(numbers);
    }
    println!("{:?}", updates);

    return updates;
}

fn get_indexes_of_correctly_sorted_updates(
    updates: &Vec<Vec<i32>>,
    sorting_rules: &Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    let mut valid_updates: Vec<Vec<i32>> = Vec::new();
    for update in updates {
        let mut is_update_good = false;
        for rule in sorting_rules {
            let left_pos = update.iter().position(|&page| page == rule[0]);
            let right_pos = update.iter().position(|&page| page == rule[1]);
            if left_pos.is_none() || right_pos.is_none(){
                continue;
            }
            println!("Rule: {:?}", rule);
            println!("{:?}|{:?}", left_pos, right_pos);
            if left_pos > right_pos {
                is_update_good = false;
                break;
            }
            is_update_good = true;
        }
        if is_update_good {
            valid_updates.push(update.clone());
        }
    }

    println!("\nValid Updates\n{:?} \n", valid_updates);

    return valid_updates;
}

fn get_middle_page_numbers_from_valid_updates(updates: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut middle_pages: Vec<i32> = Vec::new();
    for update in updates {
        middle_pages.push(update[update.len() / 2])
    }
    println!("Middle Pages: {:?}", middle_pages);
    return middle_pages;
}
