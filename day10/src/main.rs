#[path = "./utils/file.rs"]
mod file;
use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    fmt, panic, process,
    rc::Rc,
};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE", required = true)]
    file: String,
}

type PathNodeRef = Rc<RefCell<PathNode>>;

#[derive(Debug, Clone)]
struct PathNode {
    val: u32,
    up: Option<PathNodeRef>,
    down: Option<PathNodeRef>,
    left: Option<PathNodeRef>,
    right: Option<PathNodeRef>,
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
    let nodes = generate_nodes_from_file(&raw_data);
    let trailheads: Vec<Rc<RefCell<PathNode>>> = nodes
        .iter()
        .filter(|n| n.borrow().val == 0)
        .map(|n| n.clone())
        .collect();
    part_one(&trailheads);
    part_two(&trailheads);
}

fn part_one(trailheads: &Vec<Rc<RefCell<PathNode>>>) {
    let mut bfs_sum = 0;
    for trailhead in trailheads {
        let target_count = get_score_unique_destinations(trailhead.clone(), 9);
        //println!("Target Count: {:?}\n\n", target_count);
        bfs_sum += target_count;
    }
    println!("Total score: {}", bfs_sum);
}
fn part_two(trailheads: &Vec<Rc<RefCell<PathNode>>>) {
    let mut dfs_sum = 0;
    for trailhead in trailheads {
        let target_count = get_all_possible_path_score(trailhead.clone(), 9);
        //println!("Target Count: {:?}\n\n", target_count);
        dfs_sum += target_count;
    }
    println!("Total score: {}", dfs_sum)
}

fn get_all_possible_path_score(trailhead: PathNodeRef, target: u32) -> u32 {
    let mut target_count = 0;

    let mut queue = VecDeque::new();
    queue.push_back(trailhead);

    while let Some(node) = queue.pop_front() {
        //println!("Current Node: {:#?}", node.borrow().val);
        if node.borrow().val == target {
            target_count += 1;
            continue;
        }

        // Insert node to visited list

        let items = node.borrow();
        if let Some(up) = &items.up {
            if up.borrow().val == node.borrow().val + 1 {
                //println!("Up: {}", up.borrow().val);
                queue.push_front(up.clone());
            }
        }
        if let Some(down) = &items.down {
            if down.borrow().val == node.borrow().val + 1 {
                //println!("Down: {}", down.borrow().val);
                queue.push_front(down.clone());
            }
        }
        if let Some(left) = &items.left {
            if left.borrow().val == node.borrow().val + 1 {
                //println!("Left: {}", left.borrow().val);
                queue.push_front(left.clone());
            }
        }
        if let Some(right) = &items.right {
            if right.borrow().val == node.borrow().val + 1 {
                // println!("Right: {}", right.borrow().val);
                queue.push_front(right.clone());
            }
        }
    }
    return target_count;
}
fn get_score_unique_destinations(trailhead: PathNodeRef, target: u32) -> u32 {
    let mut visited = HashSet::new();
    let mut target_count = 0;

    let mut queue = VecDeque::new();
    queue.push_back(trailhead);

    while let Some(node) = queue.pop_front() {
        if visited.contains(&Rc::as_ptr(&node)) {
            continue;
        }
        //println!("Current Node: {:#?}", node.borrow().val);
        // Insert node to visited list
        visited.insert(Rc::as_ptr(&node));

        if node.borrow().val == target {
            target_count += 1;
            continue;
        }

        let items = node.borrow();
        if let Some(up) = &items.up {
            if up.borrow().val == node.borrow().val + 1 {
                //println!("Up: {}", up.borrow().val);
                queue.push_front(up.clone());
            }
        }
        if let Some(down) = &items.down {
            if down.borrow().val == node.borrow().val + 1 {
                //println!("Down: {}", down.borrow().val);
                queue.push_front(down.clone());
            }
        }
        if let Some(left) = &items.left {
            if left.borrow().val == node.borrow().val + 1 {
                //println!("Left: {}", left.borrow().val);
                queue.push_front(left.clone());
            }
        }
        if let Some(right) = &items.right {
            if right.borrow().val == node.borrow().val + 1 {
                // println!("Right: {}", right.borrow().val);
                queue.push_front(right.clone());
            }
        }
    }
    return target_count;
}

fn generate_nodes_from_file(raw_data: &String) -> Vec<PathNodeRef> {
    let mut nodes = Vec::new();
    let mut map = Vec::new();

    for line in raw_data.lines() {
        let tmp: Vec<char> = line.chars().collect();
        map.push(tmp);
    }

    for x in 0..map.len() {
        let mut temp = Vec::new();
        for y in 0..map[x].len() {
            temp.push(Rc::new(RefCell::new(PathNode {
                val: map[x][y].to_digit(10).unwrap(),
                up: None,
                down: None,
                left: None,
                right: None,
            })))
        }
        nodes.push(temp);
    }

    // add edges
    for x in 0..nodes.len() {
        for y in 0..nodes[x].len() {
            nodes[x][y].borrow_mut().up = if x > 0 {
                Some(nodes[x - 1][y].clone())
            } else {
                None
            };
            nodes[x][y].borrow_mut().down = if x < nodes.len() - 1 {
                Some(nodes[x + 1][y].clone())
            } else {
                None
            };
            nodes[x][y].borrow_mut().right = if y < nodes[x].len() - 1 {
                Some(nodes[x][y + 1].clone())
            } else {
                None
            };
            nodes[x][y].borrow_mut().left = if y > 0 {
                Some(nodes[x][y - 1].clone())
            } else {
                None
            };
            //println!("\n\nNode: {:#?}", nodes[x][y]);
        }
    }

    return nodes.into_iter().flatten().collect();
}

impl fmt::Display for PathNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.val)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
