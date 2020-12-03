use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn day2() {
    let inputs = file_to_strings("input2.txt");
    let split_input: Vec<Vec<&str>> = inputs.iter()
        .map(|line| -> Vec<&str> {line.split(|c| (c == ' ') || (c == '-')).collect()})
        .collect();
    let corrects= split_input.iter().filter(|line| correct(line)).count();
    let corrects2 = split_input.iter().filter(|line| correct2(line)).count();
    println!("{:?}", corrects);
    println!("{:?}", corrects2)
}

fn correct(args: &Vec<&str>) -> bool{
    let (min, max, letter, word) = unpack(args);
    let occurences: usize = word.chars().filter(|x| x == &letter).count();
    return (occurences >= min) && (occurences <= max)
}

fn correct2(args: &Vec<&str>) -> bool{
    let (first, second, letter, word) = unpack(args);
    return (word.chars().nth(first - 1).unwrap() == letter) != (word.chars().nth(second - 1).unwrap() == letter)
}
pub fn file_to_strings(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("file not found");
    return BufReader::new(f).lines().map(Result::unwrap).collect();
}

fn unpack(args: &Vec<&str>) -> (usize, usize, char, String) {
    let number_one : usize = args[0].parse().unwrap();
    let number_two : usize = args[1].parse().unwrap();
    let letter: char = args[2].chars().next().unwrap();
    let word: &str = args[3];
    return (number_one, number_two, letter, String::from(word))
}