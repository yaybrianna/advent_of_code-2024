#[path = "./utils/file.rs"]
mod file;

use clap::Parser;
use regex::{Captures, Regex};

use std::sync::atomic::{AtomicU64, Ordering};

static BUTTON_LIMIT: AtomicU64 = AtomicU64::new(0);

pub fn get_button_limit() -> u64 {
    BUTTON_LIMIT.load(Ordering::Relaxed)
}

pub fn set_button_limit(limit: u64) {
    BUTTON_LIMIT.store(limit, Ordering::Relaxed);
}
static UNIT_OFFSET: AtomicU64 = AtomicU64::new(0);

pub fn get_unit_offset() -> u64 {
    UNIT_OFFSET.load(Ordering::Relaxed)
}

pub fn set_unit_offset(offset: u64) {
    UNIT_OFFSET.store(offset, Ordering::Relaxed);
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE", required = true)]
    file: String,
    #[arg(short, long, value_name = "UNIT OFFSET")]
    unit_offset: Option<u64>,
    #[arg(short, long, value_name = "BUTTON LIMIT")]
    button_limit: Option<u64>,
}

#[derive(Debug, Clone, Copy)]
struct Offset {
    x: i64,
    y: i64,
}
#[derive(Debug, Clone, Copy)]
struct ClawMachineConfig {
    a_button: Offset,
    b_button: Offset,
    prize: Offset,
}

fn main() {
    let cli = Cli::parse();
    let file_path = cli.file;
    let button_limit = cli.button_limit.unwrap_or_default();
    set_button_limit(button_limit);
    let unit_offset = cli.unit_offset.unwrap_or_default();
    set_unit_offset(unit_offset);

    let raw_data = file::load_file(&file_path);
    let claw_configs = get_claw_machine_configs_from_file(&raw_data);
    let mut total_min_tokens = 0;

    //part 1
    //use flag -b 100
    //
    //part 2 -u 10000000000000
    //use flag -u 
    for config in claw_configs {
        let cost = get_min_token_cost_to_reach_prize(&config).unwrap_or_else(|| 0);

        println!("Cost: {cost}\n\n");
        total_min_tokens += cost;
    }
    println!("Total MIN cost: {total_min_tokens}");
}

fn get_min_token_cost_to_reach_prize(claw_config: &ClawMachineConfig) -> Option<i64> {
    let c_a = 3;
    let c_b = 1;

    let d_ax = claw_config.a_button.x;
    let d_ay = claw_config.a_button.y;
    let d_bx = claw_config.b_button.x;
    let d_by = claw_config.b_button.y;

    let x = claw_config.prize.x + get_unit_offset() as i64;
    let y = claw_config.prize.y + get_unit_offset() as i64;

    println!("d_ax: {d_ax}");
    println!("d_ay: {d_ay}");
    println!("d_bx: {d_bx}");
    println!("d_by: {d_by}");
    println!("x:    {x}");
    println!("y:    {y}");

    let n_a = ((d_by * x) - (d_bx * y)) / ((d_by * d_ax) - (d_bx * d_ay));
    let n_b = ((d_ay * x) - (d_ax * y)) / ((d_ay * d_bx) - (d_ax * d_by));

    println!("n_a:  {n_a}");
    println!("n_b:  {n_b}");
    println!("");

    if (n_a * d_ax) + (n_b * d_bx) != x && (n_a * d_ay) + (n_b * d_by) != y {
        return None;
    }
    let button_limit = get_button_limit();
    if button_limit > 0 && (n_a >= button_limit as i64 || n_b >= button_limit as i64) {
        return None;
    }

    let cost = (c_a * n_a) + (c_b * n_b);

    return Some(cost);
}

fn get_claw_machine_configs_from_file(file: &String) -> Vec<ClawMachineConfig> {
    let text_blocks = get_blocks_from_raw_data(&file);
    let mut claw_configs = Vec::new();

    let regex: Regex = Regex::new(r"X[+=]([0-9]*), Y[+=]([0-9]*)").unwrap();

    for block in text_blocks {
        let mut caps: Vec<Captures> = Vec::new();
        for line in block {
            caps.append(&mut regex.captures_iter(line).collect());
        }
        claw_configs.push(ClawMachineConfig {
            a_button: Offset {
                x: caps[0][1].parse::<i64>().unwrap(),
                y: caps[0][2].parse::<i64>().unwrap(),
            },
            b_button: Offset {
                x: caps[1][1].parse::<i64>().unwrap(),
                y: caps[1][2].parse::<i64>().unwrap(),
            },
            prize: Offset {
                x: caps[2][1].parse::<i64>().unwrap(),
                y: caps[2][2].parse::<i64>().unwrap(),
            },
        });
    }
    //println!("ClawMachineConfigs: {:#?}", claw_configs);
    return claw_configs;
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
