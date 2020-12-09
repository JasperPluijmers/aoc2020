use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn day9() {
    let inputs = file_to_ints("input9.txt").collect::<Vec<isize>>();
    println!("{:?}", inputs[25..].iter().enumerate()
        .filter(|(index, number)|
            !check_for_sum(&inputs[*index..(index + 25)], **number))
        .collect::<Vec<(usize, &isize)>>());
    println!("{:?}", check_continguous_sum(inputs, 14144619, 0, 0))

}

fn check_continguous_sum(numbers: Vec<isize>, requested_sum: isize, first_index: usize, second_index: usize) -> bool {
    let sum: isize = numbers[first_index..second_index].iter().sum();
    return if sum == requested_sum {
        println!("{:?}", numbers[first_index..second_index].iter().min().unwrap() + numbers[first_index..second_index].iter().max().unwrap());
        true
    } else if sum < requested_sum {
        check_continguous_sum(numbers, requested_sum, first_index, second_index + 1)
    } else {
        check_continguous_sum(numbers, requested_sum, first_index + 1, second_index)
    }
}

fn check_for_sum(numbers: &[isize], requested_sum: isize) -> bool {
    let set: HashSet<isize> = HashSet::from_iter(numbers.iter().cloned());
    return numbers.iter().map(|number| set.contains(&(requested_sum - number))).any(|x| x)
}

fn file_to_ints(filename: &str) -> impl Iterator<Item=isize> {
    let f = File::open(filename).expect("file not found");
    return BufReader::new(f).lines().map(|result| result.unwrap().parse::<isize>().unwrap());
}