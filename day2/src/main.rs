use std::fs;

const MINIMUM_VARIATION: i32 = 1;
const MAXIMUM_VARIATION: i32 = 3;

fn main() {
    // let file_path = "input.txt";
    let file_path = "test_case.txt";
    let raw_data = Box::new(load_input(&file_path));
    let mut data = Vec::new();

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
    for report in &data {
        println!("{:?}", report);
        let mut is_safe_report = false;
        let is_increasing = report[0] < report[1];
        let mut failing_level_count = 0;
        for i in 0..report.len() - 1 {
            if is_increasing && report[i] > report[i + 1] {
                println!("We changed directions, unsafe report!");
                is_safe_report = false;
                failing_level_count += 1;
                break;
            }
            let difference = if is_increasing {
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
                is_safe_report = false;
                failing_level_count += 1;
                break;
            }
            is_safe_report = true;
        }
        println!("Failing level count: {}", failing_level_count);
        if is_safe_report {
            safe_report_count += 1;
        }
    }

    println!("Total Safe Reports: {}/{}", safe_report_count, &data.len());
    // Part 2
}

fn load_input(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Unable to read file");
}
