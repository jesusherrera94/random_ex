pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

pub fn describe_number(n: i32) -> String {
    match n {
        n if n < 0 && is_even(n) => "negative even".to_string(),
        n if n < 0 && !is_even(n) => "negative odd".to_string(),
        n if n > 0 && is_even(n) => "positive even".to_string(),
        n if n > 0 && !is_even(n) => "positive odd".to_string(),
        _ => "zero".to_string(),
    }
}

pub fn main() {
    assert_eq!(describe_number(-2), "negative even");
    assert_eq!(describe_number(-1), "negative odd");
    assert_eq!(describe_number(0), "zero");
    assert_eq!(describe_number(1), "positive odd");
    assert_eq!(describe_number(2), "positive even");
    println!("All tests passed");
}