use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let file = File::open("/home/lolletsoc/devel/pets/advent_of_code/2024/day_1/input.txt").expect("File not found");

    let mut reader = BufReader::new(file);

    let mut line = String::new();
    
    let mut left = vec![];
    let mut right = vec![];
    loop {
        let num_read = reader.read_line(&mut line).expect("Reading failed");
        if num_read == 0 {break; }       
        
        let integers: Vec<i32> = line.split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect();

        left.push(integers[0]);
        right.push(integers[1]);

        line.clear();
    }

    left.sort();
    right.sort();
    
    assert!(left.len() == right.len(), "Lists must be of same size");

    let mut distance = 0;

    for i in 0..left.len() {
        let diff: i32 = left[i] - right[i];
        if diff < 0 {
            distance += diff * -1;
        } else {
            distance += diff;
        }
    }

    println!("Distance is {}", distance);

}
