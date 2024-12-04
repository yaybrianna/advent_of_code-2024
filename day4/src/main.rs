#[path = "./utils/file.rs"]
mod file;
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1
                            //
use clap::Parser;
use std::{
    fmt,
    sync::atomic::{AtomicU32, Ordering},
    thread::{self},
    usize,
};

static XMAS_COUNT: AtomicU32 = AtomicU32::new(0);

pub fn get_xmas_count() -> u32 {
    XMAS_COUNT.load(Ordering::Relaxed)
}

pub fn set_xmas_count_conditionals(count: u32) {
    XMAS_COUNT.store(count, Ordering::Relaxed);
}

pub fn add_xmas_count(count: u32) -> u32 {
    XMAS_COUNT.fetch_add(count, Ordering::SeqCst)
}

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
    let mut mut_data = Vec::new();

    for line in raw_data.lines() {
        let temp_line: Vec<char> = line.chars().collect();
        mut_data.push(temp_line);
    }
    let data = mut_data;

    println!("Input 2D Vector:\n{:?}", data);

    let xmas_count = get_xmas_count_from_matrix(&data);
    println!(
        "The Word Search Found: {} instances of the word ''XMAS'",
        xmas_count
    );
}

fn get_xmas_count_from_matrix(data: &Vec<Vec<char>>) -> u32 {
    for x in 0..data.len() {
        for y in 0..data[x].len() {
            if data[x][y] == 'X' {
                search_directions_for_remaining_word(x, y, data);
            }
        }
    }
    return get_xmas_count();
}

fn search_directions_for_remaining_word(start_x: usize, start_y: usize, data: &Vec<Vec<char>>) {
    let word_match = "MAS";
    thread::scope(|s| {
        for direction in Direction::iter() {
            let _tbuilder = thread::Builder::new()
                .name(direction.to_string())
                .spawn_scoped(s, move || {
                    let x = start_x.clone();
                    let y = start_y.clone();
                    match direction {
                        Direction::Left => {
                            println!(
                                "Dir: {}, Current Position: {},{}",
                                direction.to_string(),
                                x,
                                y
                            );
                            let mut word: String = String::from("");
                            if y > 2 {
                                for i in 1..4 {
                                    word.push(data[x][y - i]);
                                }
                                if word == word_match {
                                    add_xmas_count(1);
                                }
                                println!("Dir: {}, word: {}", direction.to_string(), word)
                            }
                        }
                        Direction::Right => {
                            println!(
                                "Dir: {}, Current Position: {},{}",
                                direction.to_string(),
                                x,
                                y
                            );
                            let mut word: String = String::from("");
                            if y <= data[x].len() - 4 {
                                for i in 1..4 {
                                    word.push(data[x][y + i]);
                                }
                                if word == word_match {
                                    add_xmas_count(1);
                                }
                                println!("Dir: {}, word: {}", direction.to_string(), word)
                            }
                        }
                        Direction::Up => {
                            println!(
                                "Dir: {}, Current Position: {},{}",
                                direction.to_string(),
                                x,
                                y
                            );
                            let mut word: String = String::from("");
                            if x > 2 {
                                for i in 1..4 {
                                    word.push(data[x - i][y]);
                                }
                                if word == word_match {
                                    add_xmas_count(1);
                                }
                                println!("Dir: {}, word: {}", direction.to_string(), word)
                            }
                        }
                        Direction::Down => {
                            println!(
                                "Dir: {}, Current Position: {},{}",
                                direction.to_string(),
                                x,
                                y
                            );
                            let mut word: String = String::from("");
                            if x <= data.len() - 4 {
                                for i in 1..4 {
                                    word.push(data[x + i][y]);
                                }
                                if word == word_match {
                                    add_xmas_count(1);
                                }
                                println!("Dir: {}, word: {}", direction.to_string(), word)
                            }
                        }
                        Direction::LeftUp => {
                            println!(
                                "Dir: {}, Current Position: {},{}",
                                direction.to_string(),
                                x,
                                y
                            );
                            let mut word: String = String::from("");
                            if x > 2 && y > 2 {
                                for i in 1..4 {
                                    word.push(data[x - i][y - i]);
                                }
                                if word == word_match {
                                    add_xmas_count(1);
                                }
                                println!("Dir: {}, word: {}", direction.to_string(), word)
                            }
                        }
                        Direction::RightUp => {
                            println!(
                                "Dir: {}, Current Position: {},{}",
                                direction.to_string(),
                                x,
                                y
                            );
                            let mut word: String = String::from("");
                            if x > 2 && y <= data[x].len() - 4 {
                                for i in 1..4 {
                                    word.push(data[x - i][y + i]);
                                }
                                if word == word_match {
                                    add_xmas_count(1);
                                }
                                println!("Dir: {}, word: {}", direction.to_string(), word)
                            }
                        }
                        Direction::LeftDown => {
                            println!(
                                "Dir: {}, Current Position: {},{}",
                                direction.to_string(),
                                x,
                                y
                            );
                            let mut word: String = String::from("");
                            if x < data.len() - 3 && y > 2 {
                                for i in 1..4 {
                                    word.push(data[x + i][y - i]);
                                }
                                if word == word_match {
                                    add_xmas_count(1);
                                }
                                println!("Dir: {}, word: {}", direction.to_string(), word)
                            }
                        }
                        Direction::RightDown => {
                            println!(
                                "Dir: {}, Current Position: {},{}",
                                direction.to_string(),
                                x,
                                y
                            );
                            let mut word: String = String::from("");
                            if x < data.len() - 3 && y < data[x].len() - 3 {
                                for i in 1..4 {
                                    word.push(data[x + i][y + i]);
                                }
                                if word == word_match {
                                    add_xmas_count(1);
                                }
                                println!("Dir: {}, word: {}", direction.to_string(), word)
                            }
                        }
                    }
                });
        }
    })
}

#[derive(Debug, EnumIter)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    LeftUp,
    RightUp,
    LeftDown,
    RightDown,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
