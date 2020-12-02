mod day1;
use day1::day1;

fn test(some: &String) {
    println!("{}", some)
}
fn main() {
    let s = String::from("hoi");
    test(&s);
    s.bytes();
}
