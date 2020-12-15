use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(part_1("0,3,6"), 436)
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part_1("1,3,2"), 1)
    }

    #[test]
    fn part1_example3() {
        assert_eq!(part_1("2,1,3"), 10)
    }

    #[test]
    fn part1_example4() {
        assert_eq!(part_1("1,2,3"), 27)
    }

    #[test]
    fn part1_example5() {
        assert_eq!(part_1("2,3,1"), 78)
    }

    #[test]
    fn part1_example6() {
        assert_eq!(part_1("3,2,1"), 438)
    }

    #[test]
    fn part1_example7() {
        assert_eq!(part_1("3,1,2"), 1836)
    }
}

pub fn day15() {
    println!("{:?}", part_2("6,4,12,1,20,0,16"));
}
fn part_2(input: &str) -> usize {
    let sequence = input.split(",").map(|value| value.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let mut seen_numbers: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut last_number: usize = 0;
    for (index, number) in sequence.iter().enumerate() {
        seen_numbers.insert(*number, (index, index));
        last_number = *number;
    }
    for i in sequence.len()..30000000 {
        match seen_numbers.get(&last_number) {
            Some(result) => {
                last_number = result.1 - result.0;
            }
            None => unreachable!()
        }
        match seen_numbers.get(&last_number) {
            Some(result) => {
                seen_numbers.insert(last_number, (result.1, i));
            }
            None => {
                seen_numbers.insert(last_number, (i, i));
            }
        }
    }
    return last_number
}

fn part_1(input: &str) -> usize {
    let sequence = input.split(",").map(|value| value.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let mut seen_numbers: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut last_number: usize = 0;
    for (index, number) in sequence.iter().enumerate() {
        let mut seen_sequence: Vec<usize> = Vec::new();
        seen_sequence.push(index);
        seen_numbers.insert(*number, seen_sequence);
        last_number = *number;
    }
    let mut number_to_speak = 0;
    for i in sequence.len()..2020 {
        if i% 10000 == 0 {
            println!("{:?}", i)
        }
        match seen_numbers.get(&last_number) {
            Some(result) => {
                if result.len() == 1 {
                    number_to_speak = 0
                } else {
                    let last_two = result.iter().rev().take(2).collect::<Vec<&usize>>();
                    number_to_speak = last_two[0] - last_two[1];
                }
            }
            None => unreachable!()
        }

        match seen_numbers.get(&number_to_speak) {
            Some(result) => {
                let mut new_result = result.clone();
                new_result.push(i);
                seen_numbers.insert(number_to_speak, new_result);
            }
            None => {
                let mut new_result: Vec<usize> = Vec::new();
                new_result.push(i);
                seen_numbers.insert(number_to_speak, new_result);
            }
        }
        last_number = number_to_speak;
    }
    return last_number
}