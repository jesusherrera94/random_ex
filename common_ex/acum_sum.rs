pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

pub fn sum_of_evens(start: i32, end: i32) -> i32 {
    let mut acum: i32 = 0;
    if start > end { return 0; }
    for num in start..=end {
        if is_even(num) {
            acum += num;
        }
    }
    acum
}

pub fn main() {
    println!("{}", sum_of_evens(1, 10));
    println!("{}", sum_of_evens(5, 5));
    println!("{}", sum_of_evens(10, 1));
}