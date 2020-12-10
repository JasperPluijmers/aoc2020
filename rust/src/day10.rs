use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;
use std::collections::HashMap;
use time::PreciseTime;


pub fn day10() {
    let inputs: Vec<usize> = file_to_ints("input/day10/input.txt").chain(Some(0)).sorted().collect();
    let differences: Vec<usize> = inputs.iter().zip(inputs.iter().skip(1)).map(|(first, second)| second - first).collect();

    println!("{:?}", (differences.iter().filter(|&&number| number == 3).count() + 1) * differences.iter().filter(|&&number| number == 1).count());
    let start = PreciseTime::now();
    println!("{:?}", differences.split(|&difference| difference == 3)
        .map(|ones| find_all_options(&(1..ones.len() + 2).collect::<Vec<usize>>(), 0))
        .product::<usize>());
    let middle = PreciseTime::now();
    println!("{:?}", find_all_options_memoization(&inputs, 0, &mut HashMap::new()));
    let end = PreciseTime::now();
    println!("{:?}", find_option_linear(inputs));
    let real_end = PreciseTime::now();
    println!("{:?}", start.to(middle));
    println!("{:?}", middle.to(end));
    println!("{:?}", end.to(real_end));
    return;
}

fn find_all_options(adapters: &Vec<usize>, last_adapter_index: usize) -> usize {
    return if last_adapter_index == adapters.len() - 1 {
        1
    } else {
        return adapters.iter().enumerate()
            .filter(|(_index, &adapter)| (adapter > adapters[last_adapter_index]) & (adapter < adapters[last_adapter_index] + 4))
            .map(|(index, _adapter)| find_all_options(adapters, index))
            .sum();
    };
}

fn find_all_options_memoization(adapters: &Vec<usize>, last_adapter_index: usize, cache: &mut HashMap<usize, usize>) -> usize {
    return if last_adapter_index == adapters.len() - 1 {
        1
    } else {
        return adapters.iter().enumerate()
            .filter(|(_index, &adapter)| (adapter > adapters[last_adapter_index]) & (adapter < adapters[last_adapter_index] + 4))
            .map(|(index, _adapter)| {
                match cache.get(&index) {
                    Some(result) => *result,
                    None => {
                        let new_paths = find_all_options_memoization(adapters, index, cache);
                        cache.insert(index, new_paths);
                        new_paths
                    }
                }
            })
            .sum();
    };
}

fn find_option_linear(adapters: Vec<usize>) -> usize {
    let mut path_lengths: HashMap<usize, usize> = HashMap::new();
    path_lengths.insert(0, 1);

    for adapter in adapters.iter() {
        for delta_index in 1..4 {
            match path_lengths.get(&(adapter + delta_index)) {
                Some(result) => path_lengths.insert(adapter + delta_index, result + path_lengths.get(&adapter).unwrap()),
                None => path_lengths.insert(adapter + delta_index, *path_lengths.get(&adapter).unwrap())
            };
        }
    }
    return *path_lengths.get(adapters.iter().max().unwrap()).unwrap()
}

fn file_to_ints(filename: &str) -> impl Iterator<Item=usize> {
    let f = File::open(filename).expect("file not found");
    return BufReader::new(f).lines().map(|result| result.unwrap().parse::<usize>().unwrap());
}