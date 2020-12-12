use regex::{Regex};
use crate::util;


pub fn day12() {
    println!("{:?}", part_1(util::file_to_strings("input/day12/input.txt")));
    println!("{:?}", part_2(util::file_to_strings("input/day12/input.txt")))
}

fn part_2(input: impl Iterator<Item=String>) -> i32 {
    let mut current_position = Position {
        x: 0, y: 0, direction: Direction::EAST
    };

    let mut waypoint = Position {
        x: 10, y: 1, direction: Direction::EAST
    };
    let command_regex = Regex::new(r"(?P<instruction>[NSEWLRF])(?P<value>\d+)").unwrap();
    for command in input {
        let mut parsed = command_regex.captures(&*command).unwrap();
        match &parsed["instruction"] {
            "N" | "S" | "E" | "W"  => {
                waypoint = move_direction(&direction_from_string(&parsed["instruction"]), (&parsed["value"]).parse().unwrap(),  waypoint);
            }
            "L" | "R" => {
                waypoint = rotate_around_ship(waypoint, &parsed["instruction"], (&parsed["value"]).parse().unwrap())
            }
            "F" => {
                current_position = Position {
                    x: (current_position.x + waypoint.x * (&parsed["value"]).parse::<i32>().unwrap()),
                    y: (current_position.y + waypoint.y * (&parsed["value"]).parse::<i32>().unwrap()),
                    ..current_position
                }
            }
            _ => panic!("Should not be possible")
        }
    }
    return current_position.x.abs() + current_position.y.abs()
}

fn rotate_around_ship(waypoint: Position, turning_to: &str, degrees: i32) -> Position {
    let amount = (degrees % 360) / 90;
    let mut new_x = waypoint.x;
    let mut new_y = waypoint.y;
    for _ in 0..amount {
        let mut old_x = new_x;
        let mut old_y = new_y;
        match turning_to {
            "R" => {
                new_y = -old_x;
                new_x = old_y;
            }
            "L" => {
                new_y = old_x;
                new_x = -old_y;
            }
            _ => {
                panic!("Invalid turning to value")
            }
        }

    };
    return Position {
        x: new_x,
        y: new_y,
        ..waypoint
    }
}

fn part_1(input: impl Iterator<Item=String>) -> i32 {
    let mut current_position = Position {
        x: 0, y: 0, direction: Direction::EAST
    };
    let command_regex = Regex::new(r"(?P<instruction>[NSEWLRF])(?P<value>\d+)").unwrap();
    for command in input {
        let mut parsed = command_regex.captures(&*command).unwrap();
        match &parsed["instruction"] {
            "N" | "S" | "E" | "W"  => {
                current_position = move_direction(&direction_from_string(&parsed["instruction"]), (&parsed["value"]).parse().unwrap(),  current_position);
            }
            "L" | "R" => {
                current_position = Position{direction: turn_degrees(&current_position.direction, &parsed["instruction"], (&parsed["value"]).parse().unwrap()), ..current_position}
            }
            "F" => {
                current_position = move_direction(&current_position.direction.clone(), (&parsed["value"]).parse().unwrap(), current_position)
            }
            _ => panic!("Should not be possible")
        }
    }
    return current_position.x.abs() + current_position.y.abs()
}

fn turn_degrees(direction: &Direction, turning_to: &str, degrees: i32) -> Direction {
    let amount = (degrees % 360) / 90;
    let mut new_direction = direction.clone();
    for i in 0..amount {
        new_direction = turn(new_direction, turning_to)
    }
    new_direction
}

impl Clone for Direction {
    fn clone(&self) -> Self {
        match self {
            Direction::NORTH => {Direction::NORTH}
            Direction::EAST => {Direction::EAST}
            Direction::SOUTH => {Direction::SOUTH}
            Direction::WEST => {Direction::WEST}
        }
    }
}

fn move_direction(direction: &Direction, value: i32, current_position: Position) -> Position {
    let mut dx = 0;
    let mut dy = 0;
    match direction {
        Direction::NORTH => dy += value,
        Direction::EAST => dx += value,
        Direction::SOUTH => dy -= value,
        Direction::WEST => dx -= value
    }
    return Position {
        x: current_position.x + dx,
        y: current_position.y + dy,
        ..current_position
    }
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    direction: Direction
}
#[derive(Debug)]
enum Direction {
    NORTH,
    WEST,
    SOUTH,
    EAST
}

fn direction_from_string(instruction: &str) -> Direction{
    match instruction {
        "N" => Direction::NORTH,
        "E" => Direction::EAST,
        "S" => Direction::SOUTH,
        "W" => Direction::WEST,
        _ => panic!("Not possible")
    }
}
fn turn(current_direction: Direction, turning_to: &str) -> Direction {
    match turning_to {
        "R" => {
        match current_direction {
            Direction::NORTH => {Direction::EAST}
            Direction::EAST => {Direction::SOUTH}
            Direction::SOUTH => {Direction::WEST}
            Direction::WEST => {Direction::NORTH}
        }
        }
        "L" => {
            match current_direction {
                Direction::NORTH => {Direction::WEST}
                Direction::WEST => {Direction::SOUTH}
                Direction::SOUTH => {Direction::EAST}
                Direction::EAST => {Direction::NORTH}
            }

        }
        _ => {
            panic!("Invalid turning to value")
        }
    }
}