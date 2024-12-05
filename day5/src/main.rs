#[path = "./utils/file.rs"]
mod file;
use clap::Parser;
use std::sync::mpsc::channel;
use std::thread;

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

    //part 1

    println!("\nPROCESSING VALID UPDATES...\n");
    let valid_updates = get_valid_updates(&updates, &ordering_rules);
    let middle_pages = get_middle_page_numbers_from_valid_updates(&valid_updates);
    let _sum = sum_middle_pages(middle_pages);

    // part 2
    println!("\nPROCESSING INVALID UPDATES...\n");
    let invalid_updates = get_invalid_updates(&updates, &ordering_rules);
    let sorted_updates = sort_invalid_updates(&invalid_updates, &ordering_rules);
    let sorted_middle_pages = get_middle_page_numbers_from_valid_updates(&sorted_updates);
    let _sum = sum_middle_pages(sorted_middle_pages);
}

fn sum_middle_pages(pages: Vec<i32>) -> i32 {
    let mut sum = 0;
    for page in pages {
        sum += page;
    }
    println!("\nMiddle Pages Sum: {}", sum);
    return sum;
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

fn get_valid_updates(updates: &Vec<Vec<i32>>, sorting_rules: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut valid_updates: Vec<Vec<i32>> = Vec::new();
    for update in updates {
        if is_valid_update(update, sorting_rules) {
            valid_updates.push(update.clone());
        }
    }

    println!("\nValid Updates\n{:?} \n", valid_updates);

    return valid_updates;
}
fn get_invalid_updates(updates: &Vec<Vec<i32>>, sorting_rules: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut invalid_updates: Vec<Vec<i32>> = Vec::new();
    for update in updates {
        if !is_valid_update(update, sorting_rules) {
            invalid_updates.push(update.clone());
        }
    }

    println!("\nInvalid Updates\n{:?} \n", invalid_updates);

    return invalid_updates;
}

fn is_valid_update(update: &Vec<i32>, sorting_rules: &Vec<Vec<i32>>) -> bool {
    let mut is_update_good = false;
    for rule in sorting_rules {
        let left_pos = update.iter().position(|&page| page == rule[0]);
        let right_pos = update.iter().position(|&page| page == rule[1]);
        if left_pos.is_none() || right_pos.is_none() {
            continue;
        }
        //println!("Rule: {:?}", rule);
        if left_pos > right_pos {
            is_update_good = false;
            break;
        }
        is_update_good = true;
    }
    return is_update_good;
}

fn sort_invalid_updates(
    invalid_updates: &Vec<Vec<i32>>,
    sorting_rules: &Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    let mut updates: Vec<Vec<i32>> = Vec::new();

    let (tx, rx) = channel();
    thread::scope(|s| {
        for update in invalid_updates {
            let tx = tx.clone();
            s.spawn(move || {
                let mut sorted_update = update.clone();
                let mut is_valid = is_valid_update(&sorted_update, sorting_rules);
                while !is_valid {
                    for rule in sorting_rules {
                        let left_pos = sorted_update.iter().position(|&page| page == rule[0]);
                        let right_pos = sorted_update.iter().position(|&page| page == rule[1]);
                        if left_pos.is_none() || right_pos.is_none() {
                            continue;
                        }
                        //println!("Rule: {:?}", rule);
                        if left_pos > right_pos {
                            sorted_update.swap(left_pos.unwrap(), right_pos.unwrap());
                        }
                        is_valid = is_valid_update(&sorted_update, sorting_rules);
                    }
                }
                println!("\n!sorted: {:?}", update);
                println!(" sorted: {:?}\n", sorted_update);
                tx.send(sorted_update).unwrap();
            });
        }
    });
    drop(tx);

    while let Ok(msg) = rx.recv() {
        updates.push(msg);
    }

    get_valid_updates(&updates, &sorting_rules);
    return updates;
}

fn get_middle_page_numbers_from_valid_updates(updates: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut middle_pages: Vec<i32> = Vec::new();
    for update in updates {
        middle_pages.push(update[update.len() / 2])
    }
    println!("Middle Pages: {:?}", middle_pages);
    return middle_pages;
}
