use crate::util;
use num::Integer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day12_part2_example1() {
        assert_eq!(part_2("input/day13/example1.txt"), 1068781)
    }

    #[test]
    fn test_day12_part2_example2() {
        assert_eq!(part_2("input/day13/example2.txt"), 3417)
    }

    #[test]
    fn test_day12_part2_example3() {
        assert_eq!(part_2("input/day13/example3.txt"), 754018)
    }

    #[test]
    fn test_day12_part2_example4() {
        assert_eq!(part_2("input/day13/example4.txt"), 779210)
    }

    #[test]
    fn test_day12_part2_example5() {
        assert_eq!(part_2("input/day13/example5.txt"), 1261476)
    }

    #[test]
    fn test_day12_part2_example6() {
        assert_eq!(part_2("input/day13/example6.txt"), 1202161486)
    }

}
pub fn day13() {
    println!("{:?}", part_1(util::file_to_strings("input/day13/example1.txt").collect::<Vec<String>>()));
    println!("{:?}", part_2("input/day13/input.txt"));

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
fn part_2(filename: &str) -> usize {
    let input = util::file_to_strings(filename).collect::<Vec<String>>();
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

fn part_2_old(input: Vec<String>) -> isize {
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
        new_start = find_first_occurences_slow(new_start, lcm, i.1, i.0);
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

fn find_first_occurences_slow(start: isize, first_number: usize, second_number: usize, offset: usize) -> isize {
    let mut first_sequence = start;
    let mut second_sequence = 0;

    while second_sequence - first_sequence != offset as isize {
        if first_sequence < second_sequence {
            first_sequence += first_number as isize
        } else {
            let mut difference = (first_sequence - second_sequence) / second_number as isize;
            if difference > 2 {
                second_sequence += (difference - 1) * second_number as isize
            } else {
                second_sequence += second_number as isize
            }
        }
    }

    return first_sequence
}