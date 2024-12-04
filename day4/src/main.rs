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

    // part 1
    let xmas_count = get_xmas_count_from_matrix(&data);
    println!("\n\nPART 1");
    println!(
        "The Word Search Found: {} instances of the word 'XMAS'",
        xmas_count
    );

    //part 2
    let x_mas_count = get_x_mas_count_from_matrix(&data);
    println!("\n\nPART 2");
    println!(
        "The X-MAS  Search Found: {} instances of an 'X-MAS'",
        x_mas_count
    );
}
fn get_x_mas_count_from_matrix(data: &Vec<Vec<char>>) -> u32 {
    let mut x_mas_count = 0;
    for x in 1..data.len() - 1 {
        for y in 1..data[x].len() - 1 {
            if data[x][y] == 'A' {
                let mut chars: [char; 4] = ['x'; 4];
                let mut has_first_leg = false;
                let mut has_second_leg = false;
                chars[0] = data[x - 1][y - 1];
                chars[1] = data[x - 1][y + 1];
                chars[2] = data[x + 1][y + 1];
                chars[3] = data[x + 1][y - 1];
                if chars[0] == 'M' && chars[2] == 'S' || chars[0] == 'S' && chars[2] == 'M' {
                    has_first_leg = true;
                }
                if has_first_leg {
                    if chars[1] == 'M' && chars[3] == 'S' || chars[1] == 'S' && chars[3] == 'M' {
                        has_second_leg = true;
                    }
                }
                if has_first_leg && has_second_leg {
                    x_mas_count += 1;
                }
            }
        }
    }
    return x_mas_count;
}
fn get_xmas_count_from_matrix(data: &Vec<Vec<char>>) -> u32 {
    let remaining_word = "MAS";
    for x in 0..data.len() {
        for y in 0..data[x].len() {
            if data[x][y] == 'X' {
                search_directions_for_remaining_word(remaining_word, x, y, data);
            }
        }
    }
    return get_xmas_count();
}

fn search_directions_for_remaining_word(
    remaining_word: &str,
    start_x: usize,
    start_y: usize,
    data: &Vec<Vec<char>>,
) {
    thread::scope(|s| {
        for direction in Direction::iter() {
            let _tbuilder = thread::Builder::new()
                .name(direction.to_string())
                .spawn_scoped(s, move || {
                    let x = start_x.clone();
                    let y = start_y.clone();

                    println!(
                        "Dir: {}, Current Position: {},{}",
                        direction.to_string(),
                        x,
                        y
                    );
                    let mut word: String = String::from("");
                    match direction {
                        Direction::Left => {
                            if y >= remaining_word.len() {
                                for i in 1..=remaining_word.len() {
                                    word.push(data[x][y - i]);
                                }
                            }
                        }
                        Direction::Right => {
                            if y < data[x].len() - remaining_word.len() {
                                for i in 1..=remaining_word.len() {
                                    word.push(data[x][y + i]);
                                }
                            }
                        }
                        Direction::Up => {
                            if x >= remaining_word.len() {
                                for i in 1..=remaining_word.len() {
                                    word.push(data[x - i][y]);
                                }
                            }
                        }
                        Direction::Down => {
                            if x < data.len() - remaining_word.len() {
                                for i in 1..=remaining_word.len() {
                                    word.push(data[x + i][y]);
                                }
                            }
                        }
                        Direction::LeftUp => {
                            if x >= remaining_word.len() && y >= remaining_word.len() {
                                for i in 1..=remaining_word.len() {
                                    word.push(data[x - i][y - i]);
                                }
                            }
                        }
                        Direction::RightUp => {
                            if x >= remaining_word.len() && y < data[x].len() - remaining_word.len()
                            {
                                for i in 1..=remaining_word.len() {
                                    word.push(data[x - i][y + i]);
                                }
                            }
                        }
                        Direction::LeftDown => {
                            if x < data.len() - remaining_word.len() && y >= remaining_word.len() {
                                for i in 1..=remaining_word.len() {
                                    word.push(data[x + i][y - i]);
                                }
                            }
                        }
                        Direction::RightDown => {
                            if x < data.len() - remaining_word.len()
                                && y < data[x].len() - remaining_word.len()
                            {
                                for i in 1..=remaining_word.len() {
                                    word.push(data[x + i][y + i]);
                                }
                            }
                        }
                    }

                    if word == remaining_word {
                        add_xmas_count(1);
                    }

                    println!(
                        "Dir: {}, found: {}, looking for: {}",
                        direction.to_string(),
                        word,
                        remaining_word
                    );
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
