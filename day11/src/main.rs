#[path = "./utils/file.rs"]
mod file;
use std::{
    borrow::BorrowMut,
    collections::{hash_map, HashMap},
    panic, process,
    sync::atomic::{AtomicI64, Ordering},
    thread::{self},
};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE", required = true)]
    file: String,
    #[arg(short, long, value_name = "DEPTH", required = true)]
    depth: usize,
}
static STONE_COUNT: AtomicI64 = AtomicI64::new(0);

pub fn add_stone_count(count: i64) -> i64 {
    STONE_COUNT.fetch_add(count, Ordering::SeqCst)
}

pub fn get_stone_count() -> i64 {
    STONE_COUNT.load(Ordering::SeqCst)
}

fn main() {
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler and exit the process
        orig_hook(panic_info);
        process::exit(1);
    }));

    let cli = Cli::parse();
    let file_path = cli.file;
    let depth = cli.depth;
    let raw_data = file::load_file(&file_path);
    let initial_arrangment = get_inital_arrangement_from_file(&raw_data);
    let mut hash_map: HashMap<i64, i64> = HashMap::new();
    for number in initial_arrangment {
        hash_map.insert(number, 1);
    }

    for _i in 0..depth {
        apply_rules2(&mut hash_map);

       // println!("eng: {:?}", hash_map);
    }
    let mut sum = 0;
    for (_k, v) in &hash_map {
        sum += v;
    }
    println!("Number of Stones: {}", sum);
}
fn apply_rules2(engravings: &mut HashMap<i64, i64>) {
    let mut temp_hash = HashMap::new();
    //println!("eng: {:?}", engravings);

    for (key, value) in engravings.to_owned() {
        if key == 0 {
            let mut count = 0;
            if let Some(temp) = temp_hash.get(&1) {
                count = *temp;
            }
            temp_hash.insert(1, value + count);
        } else if key.to_string().len() % 2 == 0 {
            let num_str = key.to_string();
            let midpoint = num_str.char_indices().count() / 2;
            let (first_str, second_str) = num_str.split_at_checked(midpoint).unwrap();
            let first = first_str.parse::<i64>().unwrap();
            let second = second_str.parse::<i64>().unwrap();
            let mut count = 0;

            if let Some(temp) = temp_hash.get(&first) {
                count = *temp;
            }
            temp_hash.insert(first, value + count);

            let mut count = 0;
            if let Some(temp) = temp_hash.get(&second) {
                count = *temp;
            }
            temp_hash.insert(second, value + count);
        } else {
            let mut count = 0;
            if let Some(temp) = temp_hash.get(&(key * 2024)) {
                count = *temp;
            }
            temp_hash.insert(key * 2024, value + count);
        }
    }
    std::mem::swap(engravings, &mut temp_hash);
}

fn apply_rules(number: i64, num_blinks: usize) {
    //print!("{}[2J", 27 as char);
    if num_blinks == 0 {
        add_stone_count(1);
        return;
    }

    //println!("Number: {}", number);
    if number == 0 {
        apply_rules(1, num_blinks - 1);
    } else if number.to_string().len() % 2 == 0 {
        let num_str = number.to_string();
        let midpoint = num_str.char_indices().count() / 2;
        let (first, second) = num_str.split_at_checked(midpoint).unwrap();
        apply_rules(first.parse::<i64>().unwrap(), num_blinks - 1);
        apply_rules(second.parse::<i64>().unwrap(), num_blinks - 1);
    } else {
        apply_rules(number * 2024, num_blinks - 1);
    }
}

fn get_inital_arrangement_from_file(file: &String) -> Vec<i64> {
    let mut initial_arrangment = Vec::new();
    for line in file.lines() {
        let nums: Vec<i64> = line
            .split(" ")
            .map(|number| number.parse::<i64>().unwrap().clone())
            .collect();
        initial_arrangment.append(&mut nums.clone());
    }
    return initial_arrangment;
}
