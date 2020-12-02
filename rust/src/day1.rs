use std::fs;

pub fn day1() {
    let filename = "./input.txt";
    let input = fs::read_to_string(filename).expect("Can't read the file");
    let split = input.lines();
    let numbers: Vec<i32> = split.map(|x| str::parse(x).expect("not an int")).collect();

    for number in numbers.iter() {
        for number_two in numbers.iter() {
            if number + number_two == 2020 {
                println!("{}", number * number_two)
            }
        }
    }

    for number in numbers.iter() {
        for number_two in numbers.iter() {
            for number_three in numbers.iter() {
                if number + number_two + number_three == 2020 {
                    println!("{}", number * number_two * number_three)
                }
            }
        }
    }
}
