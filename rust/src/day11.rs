use std::fs::File;
use std::io::{BufReader, BufRead};
use std::{thread, time};
use std::cmp::{min, max};

const DIRECTIONS: [(isize, isize); 8] = [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)];

pub fn day11() {
    let input = file_to_strings("input/day11/input.txt");
    let mut map: Vec<Vec<Tile>> = input.iter().map(|line| line.chars().map(string_to_tile).collect()).collect();
    let mut result = Some(map);
    let begin = time::Instant::now();
    while result != None {
        map = result.unwrap();
        result = evolve_map(&map);
        if result == None {
            println!("{:?}", count_occupied(&map));
        }
    }
    println!("{:?}", begin.elapsed());
    let mut map: Vec<Vec<Tile>> = input.iter().map(|line| line.chars().map(string_to_tile).collect()).collect();
    let mut result = Some(map);
    let begin2 = time::Instant::now();
    while result != None {
        map = result.unwrap();
        result = evolve_map_2(&map);
        if result == None {
            println!("{:?}", count_occupied(&map));
        }
    }
    println!("{:?}", begin2.elapsed())
}
fn count_occupied(map: &Vec<Vec<Tile>>) -> usize {
    return map.iter().flatten().filter(|tile| tile == &&Tile::OCCUPIED).count()
}
fn print_map(map: &Vec<Vec<Tile>>) {
    std::process::Command::new("clear").status().unwrap().success();
    for row in map {
        println!(" ");
        for tile in row {
            match &tile {
                Tile::FLOOR => print!(" "),
                Tile::EMPTY => print!("L"),
                Tile::OCCUPIED => print!("#")
            }
        }
    }
}
fn evolve_map(map: &Vec<Vec<Tile>>) -> Option<Vec<Vec<Tile>>> {
    let mut changed = false;
    let mut new_map: Vec<Vec<Tile>> =  Vec::new();
    for y in 0..map.len() {
        new_map.push(Vec::new());
        for x in 0..map[0].len() {
            match map[y][x] {
                Tile::FLOOR => {
                    new_map[y].push(Tile::FLOOR)
                }
                Tile::EMPTY => {
                    if count_occupied_neigbours(map, x, y) == 0 {
                        new_map[y].push(Tile::OCCUPIED);
                        if !changed {
                            changed = true
                        }
                    } else {
                        new_map[y].push(Tile::EMPTY)
                    }
                }
                Tile::OCCUPIED => {
                    if count_occupied_neigbours(map, x, y) >= 4 {
                        new_map[y].push(Tile::EMPTY);
                        if !changed {
                            changed = true
                        }
                    } else {
                        new_map[y].push(Tile::OCCUPIED)
                    }
                }
            }
        }
    }
    if changed {
        return Some(new_map)
    }
    return None
}

fn evolve_map_2(map: &Vec<Vec<Tile>>) -> Option<Vec<Vec<Tile>>> {
    let mut changed = false;
    let mut new_map: Vec<Vec<Tile>> =  Vec::new();
    for y in 0..map.len() {
        new_map.push(Vec::new());
        for x in 0..map[0].len() {
            match map[y][x] {
                Tile::FLOOR => {
                    new_map[y].push(Tile::FLOOR)
                }
                Tile::EMPTY => {
                    if get_seen_occupied_chairs(map, x, y) == 0 {
                        new_map[y].push(Tile::OCCUPIED);
                        if !changed {
                            changed = true
                        }
                    } else {
                        new_map[y].push(Tile::EMPTY)
                    }
                }
                Tile::OCCUPIED => {
                    if get_seen_occupied_chairs(map, x, y) >= 5 {
                        new_map[y].push(Tile::EMPTY);
                        if !changed {
                            changed = true
                        }
                    } else {
                        new_map[y].push(Tile::OCCUPIED)
                    }
                }
            }
        }
    }
    if changed {
        return Some(new_map)
    }
    return None
}

fn count_occupied_neigbours(map: &Vec<Vec<Tile>>, x: usize, y: usize) -> usize {
    let mut count = 0;
    let length = map.len();
    let width = map[0].len();
    for xs in max(0, (x as isize) - 1) as usize..min(width, x + 2) {
        for ys in max(0, (y as isize) - 1) as usize..min(length, y + 2) {
            if !((xs == x) & (ys == y)) & (map[ys][xs] == Tile::OCCUPIED) {
                count += 1
            }
        }
    }
    return count
}

fn get_seen_occupied_chairs(map: &Vec<Vec<Tile>>, x: usize, y: usize) -> usize {
    let mut count = 0;
    for (dx, dy) in &DIRECTIONS {
        let mut new_x: isize = x as isize + dx;
        let mut new_y: isize = y as isize + dy;
        if (new_x < 0) | (new_y < 0) {
            continue
        }
        while ((new_y as usize) < map.len()) && ((new_x as usize) < map[0].len()) {
            match map[new_y as usize][new_x as usize] {
                Tile::FLOOR => {
                    new_x += dx;
                    new_y += dy;}
                Tile::EMPTY => {break}
                Tile::OCCUPIED => {
                    count+= 1;
                    break
                }
            }
        }
    }
    return count
}

fn get_neighbours(map: &Vec<Vec<Tile>>, x: usize, y: usize) -> Vec<&Tile> {
    let mut neighbours: Vec<&Tile> = Vec::new();
    let length = map.len();
    let width = map[0].len();
    for xs in max(0, (x as isize) - 1) as usize..min(width, x + 2) {
        for ys in max(0, (y as isize) - 1) as usize..min(length, y + 2) {
            if !((xs == x) & (ys == y)) {
                neighbours.push(&map[ys][xs])
            }
        }
    }
    return neighbours
}

pub fn file_to_strings(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("file not found");
    return BufReader::new(f).lines().map(Result::unwrap).collect();
}

pub fn string_to_tile(tile_string: char) -> Tile {
    match tile_string {
        '.' => Tile::FLOOR,
        'L' => Tile::EMPTY,
        'H' => Tile::OCCUPIED,
        _ => panic!()
    }
}


#[derive(Debug, PartialEq)]
pub enum Tile {
    FLOOR,
    EMPTY,
    OCCUPIED
}