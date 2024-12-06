#[path = "./utils/file.rs"]
mod file;
use std::{collections::HashSet, mem, ops::Add, usize};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE", required = true)]
    file: String,
}
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn main() {
    let cli = Cli::parse();
    let file_path = cli.file;
    let raw_data = file::load_file(&file_path);
    let map = initilize_map(&raw_data);

    let unique_coordinates = traverse_map_get_unique_coordinates(&map);
    println!("\n\nUnique Coordinate Count: {}", unique_coordinates.len())
}

fn traverse_map_get_unique_coordinates(map: &Vec<Vec<char>>) -> HashSet<Coordinate> {
    let mut unique_coordinates: HashSet<Coordinate> = HashSet::new();

    let current_pos = get_starting_position(map);
    if current_pos.is_none() {
        println!("Something went weird with your input file, there was no starting position...")
    }
    let mut current_pos = current_pos.unwrap();
    let mut current_direction = Direction::Up;
    let mut can_exit = is_able_to_exit(&map, &current_pos, &current_direction);

    println!("Starting Position: {:?}", current_pos);

    while !can_exit {
        unique_coordinates.insert(current_pos.clone());
        can_exit = is_able_to_exit(&map, &current_pos, &current_direction);
        if can_exit {
            break;
        }
        let can_proceed = !is_facing_obsitcle(
            &map,
            &current_pos,
            &current_direction
        );
        println!("Current Position: {:?}, Can Proceed? {}", current_pos, can_proceed);
        if !can_proceed {
            current_direction =
                Direction::from_usize(((current_direction as usize) + 1) % Direction::COUNT);
            continue;
        }

        current_pos.move_direction(&current_direction);

    }

    println!("\nSUCCESSFULLY EXITED MAP");

    return unique_coordinates;
}

fn is_able_to_exit(
    map: &Vec<Vec<char>>,
    current_pos: &Coordinate,
    current_direction: &Direction,
) -> bool {
    let adjacent_edges = get_immediately_adjacent_edges(&map, &current_pos);
    println!("Adjacent Edges: {:?}", adjacent_edges);
    if adjacent_edges.len() == 0 {
        return false;
    }
    for edge in adjacent_edges {
        if edge == *current_direction {
            return true;
        }
    }
    return false;
}

fn get_immediately_adjacent_edges(
    map: &Vec<Vec<char>>,
    current_pos: &Coordinate,
) -> Vec<Direction> {
    let mut directions = Vec::new();
    if current_pos.x == 0 {
        directions.push(Direction::Up);
    }
    if current_pos.x == map.len() - 1 {
        directions.push(Direction::Down);
    }
    if current_pos.y == 0 {
        directions.push(Direction::Left);
    }
    if current_pos.y == map[0].len() - 1 {
        directions.push(Direction::Right);
    }

    return directions;
}

fn is_facing_obsitcle(map: &Vec<Vec<char>>, current_pos: &Coordinate, direction: &Direction) -> bool {
    let mut coordinate_to_check = current_pos.clone();
    coordinate_to_check.move_direction(direction);
    if map[coordinate_to_check.x][coordinate_to_check.y] == '#'{
        return true;
    }
    return false;

 }

fn get_starting_position(map: &Vec<Vec<char>>) -> Option<Coordinate> {
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if map[x][y] == '^' {
                return Some(Coordinate { x, y });
            }
        }
    }
    return None;
}

#[repr(usize)]
#[derive(Debug, EnumCountMacro, EnumIter, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn from_usize(value: usize) -> Direction {
        match value {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

impl Coordinate {
    fn move_direction(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.x -= 1,
            Direction::Right => self.y += 1,
            Direction::Down => self.x += 1,
            Direction::Left => self.y -= 1,
        }
    }
}

fn initilize_map(raw_data: &String) -> Vec<Vec<char>> {
    let mut map = Vec::new();
    for line in raw_data.lines() {
        let tmp: Vec<char> = line.chars().collect();
        map.push(tmp);
    }
    return map;
}
