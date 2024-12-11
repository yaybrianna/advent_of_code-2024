#[path = "./utils/file.rs"]
mod file;
use std::{collections::HashSet, isize, panic, process};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE", required = true)]
    file: String,
}
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Coordinate {
    x: isize,
    y: isize,
}

type Map = Vec<Vec<char>>;
type Coordinates = Vec<Coordinate>;
type Frequency = char;

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
    let map = get_map_from_file(&raw_data);
    let unique_antinodes = get_unique_antinodes(&map);
    println!("Unique Antinode Count: {}", unique_antinodes.len());
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

fn get_unique_antinodes(map: &Vec<Vec<char>>) -> HashSet<Coordinate> {
    let mut unique_antinodes = HashSet::new();
    let unique_freqs = get_all_unique_frequencies(map);
    println!("Unique Frequencies: {:?}", unique_freqs);

    for frequency in unique_freqs {
        let all_tower_coordinates = get_all_tower_locations_for_freq(map, frequency);
        /*
        println!(
            "All Tower Coordinates({}): {:?}",
            frequency, all_tower_coordinates
        );
        */
        let antinodes = get_antinodes_for_freq(&all_tower_coordinates);
        let antinodes: Coordinates = antinodes
            .iter()
            .filter(|ac| {
                ac.y >= 0 && ac.x >= 0 && ac.x < map.len() as isize && ac.y < map[0].len() as isize
            })
            .map(|ac| ac.clone())
            .collect();
        println!("Antinode Count: {}", antinodes.len());
        //print_coordinates(&map, &antinodes);

        for antinode in antinodes {
            unique_antinodes.insert(antinode);
        }
    }

    print_coordinates(&map, &unique_antinodes.clone().into_iter().collect());
    return unique_antinodes;
}

fn get_antinodes_for_freq(tower_coordinates: &Coordinates) -> Coordinates {
    let mut antinode_coordinates = Vec::new();
    for i in 0..tower_coordinates.len() {
        for j in i + 1..tower_coordinates.len() {
            let delta_x = tower_coordinates[i].x - tower_coordinates[j].x;
            let delta_y = tower_coordinates[i].y - tower_coordinates[j].y;
            println!("Delta X: {} | Delta Y: {}", delta_x, delta_y);

            let point1_x;
            let point1_y;

            let point2_x;
            let point2_y;

            if delta_x > 0 {
                //point i is further from origin than point j so add abs to it's x
                point1_x = tower_coordinates[i].x + delta_x.abs();
                point2_x = tower_coordinates[j].x - delta_x.abs();
            } else {
                point1_x = tower_coordinates[i].x - delta_x.abs();
                point2_x = tower_coordinates[j].x + delta_x.abs();
            }
            if delta_y > 0 {
                //point i is further from origin than point j so  sub abs to it's y
                point1_y = tower_coordinates[i].y + delta_y.abs();
                point2_y = tower_coordinates[j].y - delta_y.abs();
            } else {
                point1_y = tower_coordinates[i].y - delta_y.abs();
                point2_y = tower_coordinates[j].y + delta_y.abs();
            }

            antinode_coordinates.push(Coordinate {
                x: point1_x,
                y: point1_y,
            });
            antinode_coordinates.push(Coordinate {
                x: point2_x,
                y: point2_y,
            });
        }
    }
    // println!("Anitnode Coordinates: {:?}", antinode_coordinates);

    return antinode_coordinates;
}

fn get_all_unique_frequencies(map: &Map) -> HashSet<char> {
    let mut unique_freqs = HashSet::new();
    for x in 0..map.len() {
        for y in 0..map.len() {
            if map[x][y] != '.' {
                unique_freqs.insert(map[x][y]);
            }
        }
    }
    return unique_freqs;
}

fn get_all_tower_locations_for_freq(map: &Map, frequency: Frequency) -> Coordinates {
    let mut all_tower_coordinates: Vec<Coordinate> = Vec::new();
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if map[x][y] == frequency {
                all_tower_coordinates.push(Coordinate {
                    x: x as isize,
                    y: y as isize,
                });
            }
        }
    }
    return all_tower_coordinates;
}

fn print_coordinates(map: &Map, coordinates: &Coordinates) {
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if coordinates.contains(&Coordinate {
                x: x as isize,
                y: y as isize,
            }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    return;
}
