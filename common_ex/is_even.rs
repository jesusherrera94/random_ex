pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

pub fn main() {
    println!("{}", is_even(2));
    println!("{}", is_even(3));
}