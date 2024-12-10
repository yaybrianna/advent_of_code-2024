#[path = "./utils/file.rs"]
mod file;
use std::{collections::VecDeque, panic, process};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE", required = true)]
    file: String,
}
#[derive(Debug)]
struct Operation {
    total: u32,
    operands: Vec<u32>,
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
    let operations = get_operations_from_raw_data(&raw_data);
    let valid_operations = get_valid_operations(&operations);
    let calibration_total = get_total_calibaration_result(&valid_operations);
    println!("Calibration Total: {}", calibration_total);
}

fn get_total_calibaration_result(operations: &Vec<&Operation>) -> u32 {
    let mut sum = 0;
    for operation in operations {
        sum += operation.total;
    }
    return sum;
}

fn get_valid_operations(operations: &Vec<Operation>) -> Vec<&Operation> {
    let mut valid_operations = Vec::new();
    for operation in operations {
        let expressions = generate_test_expressions_operation(operation);
        for expression in expressions {
            let result = evaluate_math_expression(expression.as_str());
            if result == operation.total {
                valid_operations.push(operation);
            }
        }
    }
    return valid_operations;
}

fn generate_test_expressions_operation(operation: &Operation) -> Vec<String> {
    let mut test_expressions = Vec::new();
    let valid_operators = vec!['+', '*'];
    let num_possible_variant = (operation.operands.len() - 1) * 2;
    println!("Operands: {:?}", operation.operands);
    println!("Possible Variants: {num_possible_variant}");
    let mut char_vec: Vec<char> = Vec::new();
    get_operators_for_operand(
        'r',
        &valid_operators,
        0,
        (num_possible_variant) as u32,
        &mut char_vec,
    );

    //    println!("Operators: {:?}", operators);
    println!("{:?}", char_vec);
    return test_expressions;
}

fn get_operators_for_operand(
    valid_operator: char,
    valid_operators: &Vec<char>,
    depth: u32,
    possible_variations: u32,
    vector: &mut Vec<char>,
) {
    //println!("{:?}", vector);
    if depth == possible_variations / 2 {
        vector.push(valid_operator.clone());
        return;
    }
    if valid_operator == 'r' {
        vector.pop();
    }
    for i in 0..valid_operators.len() {
        get_operators_for_operand(
            valid_operators[i],
            valid_operators,
            depth + 1,
            possible_variations,
            vector,
        );
    }
}

fn evaluate_math_expression(expression: &str) -> u32 {
    let mut parts: VecDeque<&str> = expression.split(" ").collect();
    let mut result = 0;
    let mut current_operator = "+";
    while parts.len() > 0 {
        let part = parts.pop_front().unwrap();
        let number = part.parse::<u32>();
        if number.is_err() {
            current_operator = part;
            continue;
        }
        result = if current_operator == "+" {
            result + number.unwrap()
        } else if current_operator == "*" {
            result * number.unwrap()
        } else {
            result
        }
    }
    return result;
}

fn get_operations_from_raw_data(data: &String) -> Vec<Operation> {
    let mut operations = Vec::new();
    for line in data.lines() {
        let partitions: Vec<&str> = line.split(": ").collect();
        let total = partitions[0].parse::<u32>().unwrap();
        let operands: Vec<u32> = partitions[1]
            .split(" ")
            .map(|op| op.parse::<u32>().unwrap())
            .collect();
        operations.push(Operation { total, operands })
    }
    // println!("Operations:\n{:?}", operations);
    return operations;
}
