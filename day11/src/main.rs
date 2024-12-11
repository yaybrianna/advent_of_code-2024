#[path = "./utils/file.rs"]
mod file;
use std::{
    panic, process,
    sync::atomic::{AtomicU64, Ordering},
    thread,
};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE", required = true)]
    file: String,
}
static STONE_COUNT: AtomicU64 = AtomicU64::new(0);

pub fn add_stone_count(count: u64) -> u64 {
    STONE_COUNT.fetch_add(count, Ordering::SeqCst)
}

pub fn get_stone_count() -> u64 {
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
    let raw_data = file::load_file(&file_path);
    //let mut initial_arrangment = get_inital_arrangement_from_file(&raw_data);
    let initial_arrangment_part2 = get_inital_arrangement_from_file(&raw_data);
    //apply_rules(&mut initial_arrangment, 25);
    //println!("Number of Stones: {}", initial_arrangment.len());

    for num in initial_arrangment_part2 {
        let mut vec = vec![num];
        apply_rules(&mut vec, 25);
        add_stone_count(vec.len() as u64);
        drop(vec);
    }
    println!("Number of Stones: {}", get_stone_count());
}

fn apply_rules(initial_arrangment: &mut Vec<i64>, num_blinks: usize) {
    for _i in 0..num_blinks {
        let mut temp = Vec::new();
        for mut num in initial_arrangment.to_owned() {
            if num == 0 {
                num = 1;
                temp.push(num);
                continue;
            }
            if num.to_string().len() % 2 == 0 {
                let num_str = num.to_string();
                let midpoint = num_str.char_indices().count() / 2;
                let (first, second) = num_str.split_at_checked(midpoint).unwrap();
                //println!("First: {first}, Second: {second}");
                temp.push(first.parse::<i64>().unwrap());
                temp.push(second.parse::<i64>().unwrap());
                continue;
            }
            num = num * 2024;
            temp.push(num);
        }
        //println!("{:?}", temp);
        *initial_arrangment = temp;
    }
}


fn get_inital_arrangement_from_file(file: &String) -> Vec<i64> {
    let mut initial_arrangment = Vec::new();
    for line in file.lines() {
        let nums: Vec<i64> = line
            .split(" ")
            .map(|num| num.parse::<i64>().unwrap().clone())
            .collect();
        initial_arrangment.append(&mut nums.clone());
    }
    return initial_arrangment;
}
