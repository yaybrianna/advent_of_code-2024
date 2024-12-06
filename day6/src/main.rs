#[path = "./utils/file.rs"]
mod file;
use std::{
    collections::HashSet,
    fmt::{format},
    panic, process,
    sync::atomic::{AtomicU32, Ordering},
    thread::{self},
};
use strum::EnumCount;
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

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct DirectionalCoordinate {
    coordinate: Coordinate,
    direction: Direction,
}

static OBSTACLE_COUNT: AtomicU32 = AtomicU32::new(0);

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
    let map = initilize_map(&raw_data);

    //part 1
    let unique_coordinates = traverse_map_get_unique_coordinates(&map).unwrap();
    println!("\n\nUnique Coordinate Count: {}", unique_coordinates.len());

    //part 2
    get_obstacle_count_that_cause_loop(&map, &unique_coordinates);
    println!("\n\nSUCCESSFULLY PLACED {} OBSTACLES", get_obstacle_count());
}

fn get_obstacle_count_that_cause_loop(
    map: &Vec<Vec<char>>,
    original_unique_coordinates: &HashSet<Coordinate>,
) {
    thread::scope(|s| {
        for x in 0..map.len() {
            for y in 0..map[x].len() {
                let mut local_map = map.clone();
                if local_map[x][y] == '^'
                    || !original_unique_coordinates.contains(&Coordinate { x, y })
                {
                    continue;
                }
                local_map[x][y] = '#';
                let _thread_builder = thread::Builder::new()
                    .name(format(format_args!("{:?}", Coordinate { x, y })))
                    .spawn_scoped(s, move || {
                        println!("Spawned Thread: {:?}", thread::current().name().unwrap());

                        let option = traverse_map_get_unique_coordinates(&local_map);
                        if option.is_none() {
                            add_successful_obstacle();
                            println!("ADDED AN OBSTACLE TO THE COUNT")
                        }
                    });
            }
        }
    })
}

fn traverse_map_get_unique_coordinates(map: &Vec<Vec<char>>) -> Option<HashSet<Coordinate>> {
    let mut unique_coordinates: HashSet<Coordinate> = HashSet::new();

    let current_pos = get_starting_position(map);
    if current_pos.is_none() {
        println!("Something went weird with your input file, there was no starting position...")
    }
    let mut current_pos = current_pos.unwrap();
    let mut current_direction = Direction::Up;
    let mut can_exit = is_able_to_exit(&map, &current_pos, &current_direction);

    let mut loop_corners: Vec<DirectionalCoordinate> = Vec::new();

    println!("Starting Position: {:?}", current_pos);

    while !can_exit {
        let have_we_been_here_before = unique_coordinates.contains(&current_pos);
        unique_coordinates.insert(current_pos.clone());
        can_exit = is_able_to_exit(&map, &current_pos, &current_direction);
        if can_exit {
            break;
        }
        let can_proceed = !is_facing_obsitcle(&map, &current_pos, &current_direction);
        // println!("Current Position: {:?}, Can Proceed? {}",current_pos, can_proceed);
        if !can_proceed {
            current_direction =
                Direction::from_usize(((current_direction as usize) + 1) % Direction::COUNT);
            if have_we_been_here_before {
                loop_corners.push(DirectionalCoordinate {
                    coordinate: current_pos.clone(),
                    direction: current_direction.clone(),
                });
                //println!("Loop Corners({}): {:?}", loop_corners.len(), loop_corners);
            }
            if loop_corners.len() % 2 == 0 && loop_corners.len() > 4 {
                for i in 0..=3 {
                    if loop_corners[i] == loop_corners[i + (loop_corners.len() / 2) - 1] {
                        continue;
                    }
                    return None;
                }
            }
            continue;
        }

        current_pos.move_direction(&current_direction);
    }

    println!("\nSUCCESSFULLY EXITED MAP");

    return Some(unique_coordinates);
}

fn is_able_to_exit(
    map: &Vec<Vec<char>>,
    current_pos: &Coordinate,
    current_direction: &Direction,
) -> bool {
    let adjacent_edges = get_immediately_adjacent_edges(&map, &current_pos);
    //println!("Adjacent Edges: {:?}", adjacent_edges);
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

fn is_facing_obsitcle(
    map: &Vec<Vec<char>>,
    current_pos: &Coordinate,
    direction: &Direction,
) -> bool {
    let mut coordinate_to_check = current_pos.clone();
    coordinate_to_check.move_direction(direction);
    if map[coordinate_to_check.x][coordinate_to_check.y] == '#' {
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
#[derive(Debug, EnumCountMacro, EnumIter, Copy, Clone, Eq, PartialEq, Hash)]
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

pub fn get_obstacle_count() -> u32 {
    OBSTACLE_COUNT.load(Ordering::SeqCst)
}

pub fn set_obstacle_count(count: u32) {
    OBSTACLE_COUNT.store(count, Ordering::SeqCst);
}

pub fn add_successful_obstacle() -> u32 {
    OBSTACLE_COUNT.fetch_add(1, Ordering::SeqCst)
}
