#[path = "./utils/file.rs"]
mod file;
use std::{panic, process};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE", required = true)]
    file: String,
}
#[derive(Debug, Clone)]
struct Operation {
    total: i64,
    operands: Vec<i64>,
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
    let mut allowed_operators = Vec::new();
    //part 1
    allowed_operators.append(&mut vec!['+', '-']);
    let valid_operations = get_valid_operations(&operations, &allowed_operators);
    let calibration_total = get_total_calibaration_result(&valid_operations);
    println!("Calibration Total: {}", calibration_total);

    //part two
    allowed_operators.push('|');
    let valid_operations = get_valid_operations(&operations, &allowed_operators);
    let calibration_total = get_total_calibaration_result(&valid_operations);
    println!("Calibration Total: {}", calibration_total);
}

fn get_total_calibaration_result(operations: &Vec<Operation>) -> i64 {
    let mut sum = 0;
    for operation in operations {
        sum += operation.total;
    }
    return sum;
}
fn get_valid_operations(
    operations: &Vec<Operation>,
    allowed_operators: &Vec<char>,
) -> Vec<Operation> {
    return operations
        .iter()
        .filter(|operation| is_valid_operation(operation, allowed_operators))
        .map(|operation| operation.clone())
        .collect();
}

fn is_valid_operation(operation: &Operation, allowed_operators: &Vec<char>) -> bool {
    let mut expressions: Vec<i64> = Vec::new();

    if !operation.operands.is_empty() {
        recursively_get_expressions(
            &operation.operands[1..].to_vec(),
            operation.operands[0],
            &mut expressions,
            allowed_operators,
        );
    }
    if expressions.contains(&operation.total) {
        return true;
    }
    return false;
}

fn recursively_get_expressions(
    operands: &Vec<i64>,
    current_result: i64,
    combinations: &mut Vec<i64>,
    allowed_operators: &Vec<char>,
) {
    if operands.is_empty() {
        combinations.push(current_result);
        return;
    }

    let next_operand = &operands[0];
    let remaining = &operands[1..].to_vec();
    for operator in allowed_operators {
        if *operator == '+' {
            recursively_get_expressions(
                remaining,
                next_operand + current_result,
                combinations,
                allowed_operators,
            );
        }
        if *operator == '*' {
            recursively_get_expressions(
                remaining,
                next_operand * current_result,
                combinations,
                allowed_operators,
            );
        }
        if *operator == '|' {
            let concat = format!("{}{}", current_result, next_operand).parse::<i64>();
            recursively_get_expressions(
                remaining,
                concat.unwrap(),
                combinations,
                allowed_operators,
            );
        }
    }
    recursively_get_expressions(
        remaining,
        next_operand * current_result,
        combinations,
        allowed_operators,
    );
}

fn get_operations_from_raw_data(data: &String) -> Vec<Operation> {
    let mut operations = Vec::new();
    for line in data.lines() {
        let partitions: Vec<&str> = line.split(": ").collect();
        let total = partitions[0].parse::<i64>().unwrap();
        let operands: Vec<i64> = partitions[1]
            .split(" ")
            .map(|op| op.parse::<i64>().unwrap())
            .collect();
        operations.push(Operation { total, operands })
    }
    // println!("Operations:\n{:?}", operations);
    return operations;
}
