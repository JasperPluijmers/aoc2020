use std::{fs};
use regex::Regex;
use std::collections::HashMap;

pub fn day16() {
    println!("{:?}", part_1("input/day16/input.txt"));
    println!("{:?}", part_2("input/day16/input.txt"));
}


fn part_2(filename: &str) -> usize {
    let input = fs::read_to_string(filename).unwrap();
    let input_parts = input.split("\n\n").collect::<Vec<&str>>();
    let rules = get_rules(input_parts[0]);
    let mut total = 0;
    let mut valid_tickets: Vec<Vec<usize>> = Vec::new();
    for ticket in input_parts[2].split("\n").skip(1) {
        match invalid_number(ticket, &rules) {
            Some(result) => {
                total += result
            },
            None => valid_tickets.push(ticket.split(",").map(|value| value.parse::<usize>().unwrap()).collect())
        }
    }
    let mapping = eliminate(find_validities(valid_tickets, &rules));
    let reverse_mapping = mapping.iter().enumerate().filter(|(index, &value)| value < 6).map(|(index, value)| index);
    let my_ticket: Vec<usize> = input_parts[1].split("\n").skip(1).next().unwrap().split(",").map(|value| value.parse::<usize>().unwrap()).collect();
    let mut product = 1;
    for i in reverse_mapping {
        product *= my_ticket[i]
    }

    return product;
}

fn eliminate(validities: Vec<Vec<usize>>) -> Vec<usize> {
    let mut checked: Vec<usize> = Vec::new();
    let mut old_validities = validities.clone();
    while old_validities.iter().map(|validity| validity.len()).any(|length| length != 1) {
        let mut new_validities: Vec<Vec<usize>> = Vec::new();
        for (index, validity) in old_validities.iter().enumerate() {
            if (validity.len() == 1) & (!checked.contains(&validity[0])) {
                for (other_index, new_validity) in old_validities.iter().enumerate() {
                    if index != other_index {
                        new_validities.push(new_validity.iter().filter(|value| value != &&validity[0]).map(|value| *value).collect::<Vec<usize>>())
                    } else {
                        new_validities.push(new_validity.clone())
                    }
                }
                checked.push(validity[0]);
                old_validities = new_validities.clone();
                break
            }
        }
    }
    return old_validities.iter().map(|value| value[0]).collect()
}


fn find_validities(tickets: Vec<Vec<usize>>, rules: &Vec<((usize, usize), (usize, usize))>) -> Vec<Vec<usize>> {
    let mut validities: Vec<Vec<bool>> = Vec::new();
    for _i in 0..tickets[0].len() {
        validities.push(vec![true; rules.len()])
    }
    for ticket in tickets.iter() {
        for (field_index, field) in ticket.iter().enumerate() {
            for (rules_index, ((min1, max1), (min2, max2))) in rules.iter().enumerate() {
                if !(((field >= min1) & (field <= max1)) | ((field >= min2) & (field <= max2))) {
                    validities[field_index][rules_index] = false;
                }
            }
        }

    }
    return validities.iter()
        .map(|vec| vec
            .iter()
            .enumerate()
            .filter(|(index, value)| **value)
            .map(|(index, value)| index)
            .collect::<Vec<usize>>())
        .collect();
}

fn part_1(filename: &str) -> usize {
    let input = fs::read_to_string(filename).unwrap();
    let input_parts = input.split("\n\n").collect::<Vec<&str>>();
    let rules = get_rules(input_parts[0]);
    let mut total = 0;
    for ticket in input_parts[2].split("\n").skip(1) {
        match invalid_number(ticket, &rules) {
            Some(result) => {
                total += result
            },
            None => continue
        }

    }


    return total;
}

fn invalid_number(ticket: &str, rules: &Vec<((usize, usize), (usize, usize))>) -> Option<usize> {
    for entry in ticket.split(",") {
        let number = entry.parse::<usize>().unwrap();
        let mut valid = false;
        for ((min1, max1), (min2, max2)) in rules.iter() {
            if ((&number >= min1) & (&number <= max1)) | ((&number >= min2) & (&number <= max2)) {
                valid = true
            }

        }
        if !valid {
            return Some(number)
        }
    }
    return None
}

fn get_rules(rules_input: &str) -> Vec<((usize, usize), (usize, usize))> {
    let inputs = rules_input.split("\n");
    let mut rules: Vec<((usize, usize), (usize, usize))> = Vec::new();
    let pattern = Regex::new(r"(?P<first>\d+)-(?P<second>\d+) or (?P<third>\d+)-(?P<fourth>\d+)").unwrap();

    for input in inputs {
        let matches = pattern.captures(input).unwrap();
        rules.push((
            (matches["first"].parse::<usize>().unwrap(), matches["second"].parse::<usize>().unwrap()),
            (matches["third"].parse::<usize>().unwrap(), matches["fourth"].parse::<usize>().unwrap())
        )
        );

    }
    return rules;
}