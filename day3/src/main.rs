use regex::{Captures, Regex};
use std::{env, fs};

#[derive(Debug)]
struct Multiplication {
    operands: Vec<i32>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Using file: {}", file_path);
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
    let regex: Regex = Regex::new(r"mul\(([0-9]*,[0-9]*)\)").unwrap();
    let mut multiplications: Vec<Multiplication> = Vec::new();
    let rfind: Vec<Captures> = regex.captures_iter(data).collect();
    for i in 0..rfind.len() {
        multiplications.push(Multiplication {
            operands: rfind[i][1].to_string().split(",").map(|n| n.parse::<i32>().unwrap()).collect(),
        });
        println!("{:#?}",multiplications);
    }
    return multiplications;
}

fn compute_multiplications(multiplications: &Vec<Multiplication>)-> Vec<i32>{
    let mut products = Vec::new(); 
    for multiplication in multiplications{
        let mut product = 1;
        for operand in &multiplication.operands{
            product *= operand;
        }
        products.push(product);
    }
    println!("{:#?}", products);
    return products;
}

fn compute_sum(products: &Vec<i32>) -> i32 {
    let mut total = 0;
    for product in products{
        total += product;
    }
    return  total;
}
