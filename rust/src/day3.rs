use std::fs::File;
use std::io::{BufReader, BufRead};

const SLOPES: [(usize, usize); 5] = [(1,1), (3,1), (5,1), (7,1), (1,2)];

pub fn day3() {
    let input: Vec<String> = file_to_strings("input3.txt");
    let map: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    let trees: usize = SLOPES.iter().map(|slope| calculate_trees(&map, slope.0, slope.1)).product();
    println!("{:?}", trees);
}

fn calculate_trees(map: &Vec<Vec<char>>, x_slope: usize, y_slope: usize) -> usize {
    let mut trees: usize = 0;
    let mut x_value = 0;
    for y_value in  (0..map.len()).step_by(y_slope) {
        if map[y_value][x_value] == '#' {
            trees += 1;
        }
        x_value = (x_value + x_slope) % map[0].len()
    }
    return trees
}

pub fn file_to_strings(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("file not found");
    return BufReader::new(f).lines().map(Result::unwrap).collect();
}