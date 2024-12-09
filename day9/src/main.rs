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
    let layouts = generate_file_system_layouts_from_disk_maps(&raw_data);

    //part_one
    // let fragmented_layouts = generate_fragmented_drive_layouts(&layouts);
    // let checksums = calculate_fragmented_layouts_checksums(&fragmented_layouts);

    //part two
    let contigious_file_frag = generate_file_contiguous_fragmented_drive_layouts(&layouts);
    let contigous_checksum = calculate_fragmented_layouts_checksums(&contigious_file_frag);
}

fn generate_file_system_layouts_from_disk_maps(disk_maps: &String) -> Vec<Vec<i32>> {
    let mut layouts = Vec::new();
    let mut file_id = 0;

    for line in disk_maps.lines() {
        let mut char_vec = Vec::new();
        for i in 0..line.len() {
            let value = line.chars().nth(i).unwrap().to_digit(10).unwrap();
            if i % 2 == 0 {
                for _j in 0..value {
                    let char = file_id;
                    //println!("Current Value: {:?}", char);
                    char_vec.push(char);
                }
                file_id += 1;
            } else {
                for _j in 0..value {
                    char_vec.push(-1);
                }
            }
        }
        file_id = 0;
        layouts.push(char_vec);
    }
    print_disk_layouts(&layouts);
    //println!("Input Data Length: {}", disk_maps.len());
    return layouts;
}

fn generate_fragmented_drive_layouts(drive_layouts: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut layouts = Vec::new();

    for drive_layout in drive_layouts {
        let mut fragmented_layout = drive_layout.clone();
        for i in (0..fragmented_layout.len()).rev() {
            // print_disk_layout(&fragmented_layout);
            let value = drive_layout[i];
            let is_num = value > -1;
            let first_freespace = fragmented_layout.iter().position(|&c| c == -1).unwrap();
            if first_freespace >= i {
                break;
            }
            if is_num {
                fragmented_layout.swap(i, first_freespace);
            }
        }
        layouts.push(fragmented_layout);
    }
    print_disk_layouts(&layouts);
    return layouts;
}

fn generate_file_contiguous_fragmented_drive_layouts(
    drive_layouts: &Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    let mut layouts = Vec::new();

    for drive_layout in drive_layouts {
        let mut fragmented_layout = drive_layout.clone();
        let num_files = get_max_file_id_from_layout(&fragmented_layout);
        //println!("Number of Files: {num_files}");
        for i in (0..=num_files).rev() {
            let file: Vec<usize> = fragmented_layout
                .iter()
                .enumerate()
                .filter_map(|(index, &byte)| if byte == i { Some(index) } else { None })
                .collect::<Vec<_>>();
            let file_len = file.len();
            let freespace = get_first_size_of_file_freespace_chunk(&fragmented_layout, file_len);
            if freespace.is_none() {
                continue;
            }
            let freespace = freespace.unwrap();
           // println!("File ID: {:?}", i);
           // println!("Freespace: {:?}", freespace);
           //println!("File_data: {:?}\n", file);
            if freespace[0] > file[0] {
                continue;
            }
            for j in 0..file_len {
                fragmented_layout.swap(file[j], freespace[j]);
            }
        }
        layouts.push(fragmented_layout);
    }
    print_disk_layouts(&layouts);
    return layouts;
}

fn get_first_size_of_file_freespace_chunk(
    drive_layout: &Vec<i32>,
    file_size: usize,
) -> Option<Vec<usize>> {
    let mut free_bytes = Vec::new();
    for i in 0..drive_layout.len() {
        let byte = drive_layout[i];
        if byte > -1 {
            free_bytes.clear();
            continue;
        }

        if byte == -1 {
            free_bytes.push(i);
        }

        if free_bytes.len() == file_size {
            return Some(free_bytes);
        }
    }
    return None;
}

fn get_max_file_id_from_layout(drive_layout: &Vec<i32>) -> i32 {
    let mut max = 0;
    for byte in drive_layout {
        if *byte > max {
            max = *byte;
        }
    }
    return max;
}

fn print_disk_layouts(disk_layout: &Vec<Vec<i32>>) {
    for line in disk_layout {
        print_disk_layout(&line);
    }
}

fn print_disk_layout(disk_layout: &Vec<i32>) {
    for char in disk_layout {
        if *char == -1 {
            print!(". ");
        } else {
            print!("{char} ");
        }
    }
    print!("\n");
}

fn calculate_fragmented_layouts_checksums(fragmented_layouts: &Vec<Vec<i32>>) -> Vec<u64> {
    let mut checksums = Vec::new();
    for layout in fragmented_layouts {
        let mut checksum = 0;
        for i in 0..layout.len() {
            if layout[i] == -1 {
                continue;
            }
            let value = layout[i];
            checksum += value as u64 * i as u64;
        }
        println!("Checksum: {:?}", checksum);
        checksums.push(checksum);
    }
    return checksums;
}
