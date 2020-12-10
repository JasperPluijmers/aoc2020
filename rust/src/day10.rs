use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;

pub fn day10() {
    let inputs: Vec<usize> = file_to_ints("input/day10/input.txt").chain(Some(0)).sorted().collect();
    let differences: Vec<usize> = inputs.iter().zip(inputs.iter().skip(1)).map(|(first, second)| second - first).collect();
    println!("{:?}", (differences.iter().filter(|&&number| number == 3).count() + 1) * differences.iter().filter(|&&number| number == 1).count());
    println!("{:?}", differences.split(|&difference| difference == 3).map(|ones| find_all_options(&(1..ones.len() + 2).collect::<Vec<usize>>(), 1, 0)).product::<usize>());
    return
}

fn find_all_options(adapters: &Vec<usize>, count: usize, last_adapter_index: usize) -> usize {
    return if last_adapter_index == adapters.len() - 1 {
        count
    } else {
        let options: Vec<(usize, &usize)> = adapters.iter().enumerate().filter(|(index, &adapter)| (adapter > adapters[last_adapter_index]) & (adapter < adapters[last_adapter_index] + 4)).collect();
        options.iter().map(|(index, _adapter)| find_all_options(adapters, count, *index)).sum()
    }

}

fn file_to_ints(filename: &str) -> impl Iterator<Item=usize> {
    let f = File::open(filename).expect("file not found");
    return BufReader::new(f).lines().map(|result| result.unwrap().parse::<usize>().unwrap());
}