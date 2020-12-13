use crate::util;
use num::Integer;

pub fn day13() {
    println!("{:?}", part_1(util::file_to_strings("input/day13/example1.txt").collect::<Vec<String>>()));
    println!("{:?}", part_2(util::file_to_strings("input/day13/input.txt").collect::<Vec<String>>()));

    return
}

fn part_1(input: Vec<String>) -> usize {
    let current_time: usize = input[0].parse().unwrap();
    let bus = input[1]
        .split(",")
        .filter(|&time| time != "x")
        .map(|time| time.parse::<usize>().unwrap())
        .map(|time| (time, (time - current_time%time)))
        .min_by(|a,b| a.1.cmp(&b.1)).unwrap();
    return bus.0 * bus.1
}
fn part_2(input: Vec<String>) -> usize {
    let bus: Vec<(usize, usize)> = input[1]
        .split(",")
        .enumerate()
        .filter(|(_index, number)| number != &"x")
        .map(|(index, number)| (index, number.parse::<usize>().unwrap()))
        .collect();
    let mut new_start = 0;
    let mut lcm = bus[0].1;
    let mut offset  = 0;
    for i in bus.iter() {
        offset = i.0 - offset;
        new_start = find_first_occurences(new_start,i.1 , lcm, i.0);
        lcm = lcm.lcm(&i.1);
    }
    return new_start
}
fn find_first_occurences(start: usize, new_bus_period: usize, period: usize, offset: usize) -> usize {
    let mut second_sequence = start;
    while (second_sequence + offset) % new_bus_period != 0 {
        second_sequence += period
    }
    return second_sequence
}