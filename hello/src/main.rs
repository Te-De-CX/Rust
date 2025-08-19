use std::vec;

fn main() {
    println!("Hello, world!");
    // #[warn(unused_variables)]
    let mut string: String = String::from("Hello World");
    println!("{}", string);
    let slice = &string[..6];
    println!("{}", slice);
    string.push_str(" bob");
    print!("string {}", string);
    let vec: Vec<i64> = vec![1, 2, 3, 4, 5];

    let num: i32 = 23;
    if num > 3 {
        println!("num {} is greater than 3", num)
    }
    let mut _number = 32;
    _number = 3;
    println!("{}", sum(2, 4))
}

pub fn sum(a: i32, b: i32) -> i32 {
    let digit: i32 = a + b;
    return digit;
}
