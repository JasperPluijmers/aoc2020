use crate::util;
use std::collections::HashMap;
use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(part_1("input/day14/example1.txt"), 165)
    }

    #[test]
    fn part1_input() {assert_eq!(part_1("input/day14/input.txt"), 11501064782628) }


    #[test]
    fn part2_example1() {
        assert_eq!(part_2("input/day14/example2.txt"), 208)
    }


    #[test]
    fn part2_input() {
        assert_eq!(part_2("input/day14/input.txt"), 5142195937660)
    }
}

fn part_2(filename: &str) -> usize {
    let input = util::file_to_strings(filename);

    let mut memory: HashMap<usize, usize> = HashMap::new();
    let pattern = Regex::new(r"mem\[(?P<location>\d+)] = (?P<value>\d+)").unwrap();
    let mut mask: Vec<(usize, String)> = Vec::new();
    for instruction in input {
        if instruction.starts_with("mask") {
            mask = instruction.split("")
                .skip(8)
                .enumerate()
                .filter(|(_index, value)| (value == &"X") | (value == &"1") | (value == &"1"))
                .map(|(index, value)| (index, value.to_string()))
                .collect::<Vec<(usize, String)>>();
        } else {
            let parsed = pattern.captures(&instruction).unwrap();
            let base_memory = to_binary_vector(parsed["location"].parse::<usize>().unwrap());
            let mut locations: Vec<Vec<usize>> = Vec::new();
            locations.push(base_memory);

            for mask_instruction in &mask {
                let mut new_locations: Vec<Vec<usize>> = Vec::new();
                for location in locations {
                    if mask_instruction.1 == "1" {
                        let mut new_location = location.clone();
                        new_location[mask_instruction.0] = 1;
                        new_locations.push(new_location)
                    } else if mask_instruction.1 == "X" {
                        for i in 0..2 {
                            let mut new_location = location.clone();
                            new_location[mask_instruction.0] = i;
                            new_locations.push(new_location)
                        }

                    }
                }
                locations = new_locations;
            }

            for location in locations {
                memory.insert(from_binary_vector(&location), parsed["value"].parse::<usize>().unwrap());
            }
        }
    }
    return memory.values().sum()
}

fn part_1(filename: &str) -> usize {
    let input = util::file_to_strings(filename);
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let pattern = Regex::new(r"mem\[(?P<location>\d+)] = (?P<value>\d+)").unwrap();
    let mut mask: Vec<(usize, usize)> = Vec::new();
    for instruction in input {
        if instruction.starts_with("mask") {
            mask = instruction.split("")
                .skip(8)
                .enumerate()
                .filter(|(_index, value)| (value == &"0") | (value == &"1"))
                .map(|(index, value)| (index, value.parse::<usize>().unwrap()))
                .collect::<Vec<(usize, usize)>>();
        } else {
            let parsed = pattern.captures(&instruction).unwrap();
            let mut value_vec = to_binary_vector(parsed["value"].parse::<usize>().unwrap());
            for mask_instruction in &mask {
                value_vec[mask_instruction.0] = mask_instruction.1
            }
            memory.insert(parsed["location"].parse::<usize>().unwrap(), from_binary_vector(&value_vec));
        }
    }
    return memory.values().sum()
}

fn to_binary_vector(number: usize) ->Vec<usize> {
    return format!("{:036b}", number)
        .split("")
        .filter(|value| value != &"")
        .map(|value| value.parse::<usize>().unwrap())
        .collect();
}
fn from_binary_vector(bits: &Vec<usize>) -> usize {
    bits.iter()
        .fold(0, |result, &bit| {
            (result << 1) ^ bit
        })
}