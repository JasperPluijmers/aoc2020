use std::fs;

pub fn day1() {
    let filename = "./input.txt";
    let input = fs::read_to_string(filename).expect("Can't read the file");
    let split = input.lines();
    let numbers: Vec<i32> = split.map(|x| str::parse::<i32>(x).expect("not an int")).collect();

    for number in &numbers {
        for number_two in &numbers {
            if number + number_two == 2020 {
                println!("{}", number * number_two)
            }
        }
    }

    for number in &numbers {
        for number_two in &numbers {
            for number_three in &numbers {
                if number + number_two + number_three == 2020 {
                    println!("{}", number * number_two * number_three)
                }
            }
        }
    }
}
