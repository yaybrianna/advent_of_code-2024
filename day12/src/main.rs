#[path = "./utils/file.rs"]
mod file;

use std::{char, collections::HashSet};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE", required = true)]
    file: String,
}
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Coordinate {
    value: char,
    x: usize,
    y: usize,
}

type Map = Vec<Vec<char>>;

fn main() {
    let cli = Cli::parse();
    let file_path = cli.file;
    let raw_data = file::load_file(&file_path);
    let map = get_map_from_file(&raw_data);
    let unique_names = get_unique_region_names(&map);
    let all_coords_by_region_name = get_all_regions_from_map(&map, &unique_names);
    for blah in all_coords_by_region_name {
        print_region(&blah, &map);
    }
}

fn get_all_regions_from_map(
    map: &Map,
    unique_region_names: &HashSet<char>,
) -> Vec<Vec<Coordinate>> {
    let mut vec = Vec::new();
    for name in unique_region_names {
        let mut coords = Vec::new();
        for x in 0..map.len() {
            for y in 0..map[x].len() {
                if map[x][y] == *name {
                    coords.push(Coordinate {
                        value: map[x][y],
                        x,
                        y,
                    });
                }
            }
        }
        vec.push(coords);
    }
    return vec;
}
fn get_unique_region_names(map: &Map) -> HashSet<char> {
    let mut hash_set = HashSet::new();
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            hash_set.insert(map[x][y]);
        }
    }

    return hash_set;
}

fn get_map_from_file(file: &String) -> Vec<Vec<char>> {
    let mut map = Vec::new();
    for line in file.lines() {
        let mut line_vec = Vec::new();
        for char in line.chars() {
            line_vec.push(char);
        }
        map.push(line_vec);
    }
    return map;
}

fn print_region(region: &Vec<Coordinate>, map: &Map) {
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if region.contains(&Coordinate {
                value: region[0].value,
                x,
                y,
            }) {
                print!("{}",region[0].value);
            } else {
                print!(".")
            }
        }
        print!("\n");
    }
    print!("\n");
}

fn calculate_area(coordinates: &Vec<Coordinate>) -> u64 {
    let mut area = 0;
    let xs = coordinates.iter().map(|c| c.x).collect::<usize>();



    return area;
}
