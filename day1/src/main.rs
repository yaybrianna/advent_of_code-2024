use std::fs;

struct Similarity{
    list_a_number: i32,
    list_b_count: i32
}

fn main() {
    let file_path = "./source_data/input.txt";
    //let file_path = "./source_data/test_case.txt";
    let data = Box::new(load_input(&file_path));
    let mut list_a = Vec::new();
    let mut list_b = Vec::new();
    let mut list_c = Vec::new();

    let mut similarity_scores: Vec<Similarity> = Vec::new();

    let mut total_distance = 0;
    let mut total_similarity_score = 0;
    for line in data.lines() {
        println!("{}", line);
       let temp_line: Vec<i32> = line.split(' ').filter(|str| !str.is_empty()).map(|num| num.trim().parse::<i32>().unwrap()).collect();
       println!("{:?}", temp_line);
       if temp_line.len() > 4 {
        println!("Something went wrong and there are too many substrings per line...");
       }
       list_a.push(temp_line[0]);
       list_b.push(temp_line[1]);
    }

    // Part 1
    list_a.sort();
    list_b.sort();
    for i in 0..list_a.len() { 
        if list_a[i] > list_b[i] {
            list_c.push(list_a[i] - list_b[i]);
        }
        else {
            list_c.push(list_b[i] - list_a[i]);
        }
    }

    for num in list_c {
        total_distance += num;
    }
    println!("Total distance {}", total_distance);



    // Part 2
    for num in list_a {
        let count = list_b.iter().filter(|&&numb| numb == num).count() as i32;
        similarity_scores.push(Similarity {list_a_number: num, list_b_count: count});
    }

    for score in similarity_scores{
        total_similarity_score += score.list_a_number * score.list_b_count
    }
    println!("Total Similarity Score {}", total_similarity_score);


    
}

fn load_input(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Unable to read file");
}

