use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn day2() {
    let inputs = file_to_strings("input2.txt");
    let split_input: Vec<Vec<&str>> = inputs.iter()
        .map(|line| -> Vec<&str> {line.split(|c| (c == ' ') || (c == '-')).collect()})
        .collect();
    let corrects= split_input.iter().map(|line| correct(line)).filter(|&x| x).count();
    let corrects2 = split_input.iter().map(|line| correct2(line)).filter(|&x| x).count();
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
    let number_one : usize = args.get(0).unwrap().parse().unwrap();
    let number_two : usize = args.get(1).unwrap().parse().unwrap();
    let letter: char = args.get(2).unwrap().chars().next().unwrap();
    let word: &str = args.get(3).unwrap();
    return (number_one, number_two, letter, String::from(word))
}