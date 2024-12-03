use clap::Parser;
use regex::{Captures, Regex};
use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
struct Multiplication {
    operands: Vec<i32>,
}

static PARSE_CONDITIONALS: AtomicBool = AtomicBool::new(false);

pub fn get_parse_conditionals() -> bool {
    PARSE_CONDITIONALS.load(Ordering::Relaxed)
}

pub fn set_parse_conditionals(bool: bool) {
    PARSE_CONDITIONALS.store(bool, Ordering::Relaxed);
}

fn main() {
    let cli = Cli::parse();
    let file_path = cli.file;
    set_parse_conditionals(cli.parse_conditionals);
    let raw_data = load_input(&file_path);
    let multiplications = get_multiplications_from_string(&raw_data);
    let products = compute_multiplications(&multiplications);
    let total = compute_sum(&products);

    println!("Total memory saved: {}", total)
}

fn load_input(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Unable to read file");
}

fn get_multiplications_from_string(data: &String) -> Vec<Multiplication> {
    let parse_conditionals = get_parse_conditionals();
    let regex_pattern = if parse_conditionals {
        r"mul\(([0-9]*,[0-9]*)\)|do\(\)|don[']t\(\)"
    } else {
        r"mul\(([0-9]*,[0-9]*)\)"
    };
    let regex: Regex = Regex::new(regex_pattern).unwrap();
    let mut multiplications: Vec<Multiplication> = Vec::new();
    let captured_instructions: Vec<Captures> = regex.captures_iter(data).collect();
    println!("{:#?}", &captured_instructions);

    let mut are_processing_mul = true;
    for i in 0..captured_instructions.len() {
        if captured_instructions[i][0].contains("mul") && are_processing_mul {
            multiplications.push(Multiplication {
                operands: captured_instructions[i][1]
                    .to_string()
                    .split(",")
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect(),
            });
        }
        if captured_instructions[i][0].contains("do()") {
            println!("Processing 'mul' operations");
            are_processing_mul = true;
        }
        if captured_instructions[i][0].contains("don't()") {
            println!("Skipping 'mul' operations");
            are_processing_mul = false;
        }
    }

    println!("{:#?}", multiplications);
    return multiplications;
}

fn compute_multiplications(multiplications: &Vec<Multiplication>) -> Vec<i32> {
    let mut products = Vec::new();
    for multiplication in multiplications {
        let mut product = 1;
        for operand in &multiplication.operands {
            product *= operand;
        }
        products.push(product);
    }
    println!("{:#?}", products);
    return products;
}

fn compute_sum(products: &Vec<i32>) -> i32 {
    let mut total = 0;
    for product in products {
        total += product;
    }
    return total;
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE", required = true)]
    file: String,
    #[arg(long)]
    parse_conditionals: bool,
}