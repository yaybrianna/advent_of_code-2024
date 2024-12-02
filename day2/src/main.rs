use std::{fs, env};

const MINIMUM_VARIATION: i32 = 1;
const MAXIMUM_VARIATION: i32 = 3;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Using file: {}", file_path);
    let raw_data = Box::new(load_input(&file_path));
    let mut data = Vec::new();
    let mut failed_report_indexes = Vec::new();
    // parse input file into 2D vec
    for line in raw_data.lines() {
        let temp_line: Vec<i32> = line
            .split(' ')
            .filter(|str| !str.is_empty())
            .map(|num| num.trim().parse::<i32>().unwrap())
            .collect();
        data.push(temp_line);
    }

    let mut safe_report_count = 0;
    // Part 1
    for i in 0..data.len() {
        println!("{:?}", &data[i]);
        let is_safe = is_safe_report(&data[i]);
        if is_safe {
            safe_report_count += 1;
        } else {
            failed_report_indexes.push(i);
        }
    }

    println!("Total Safe Reports: {}/{}", safe_report_count, &data.len());
    // Part 2

    println!("\nFailed Report Numbers: {:?}\n", failed_report_indexes);
    println!("INITIALIZING PROBLEM DAMPENER (tm)...\n");
    for report_index in failed_report_indexes {
        for i in 0..data[report_index].len() {
            let mut short_report = data[report_index].clone();
            short_report.remove(i);
            let is_safe = is_safe_report(&short_report);
            if is_safe {
                safe_report_count += 1;
                break;
            }
        }
    }
    println!(
        "Total Safe Reports with Problem Dampener(tm): {}/{}",
        safe_report_count,
        &data.len()
    );
}

fn load_input(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Unable to read file");
}

fn is_safe_report(report: &Vec<i32>) -> bool {
    println!("{:?}", report);
    let starts_increasing = report[0] < report[1];
    for i in 0..report.len() - 1 {
        if (starts_increasing && report[i] > report[i + 1])
            || (!starts_increasing && report[i + 1] > report[i])
        {
            println!("We changed directions, unsafe report!");
            return false;
        }
        let difference = if starts_increasing {
            report[i + 1] - report[i]
        } else {
            report[i] - report[i + 1]
        };
        if difference < MINIMUM_VARIATION || difference > MAXIMUM_VARIATION {
            println!(
                    "The difference between index {} and index {}: {} is outside the allowed range, unsafe report",
                    i,
                    i + 1,
                    difference
                );
            return false;
        }
    }
    return true;
}
