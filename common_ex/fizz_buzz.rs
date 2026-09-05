pub fn fizz_buzz(num: u32) -> String {
    if num % 3 == 0 && num % 5 == 0 {
        "FizzBuzz".to_string()
    } else {
        if num % 3 == 0  {
        "Fizz".to_string()
        } else if num % 5 == 0 {
            "Buzz".to_string()
        } else {
            num.to_string()
        }
    }
}

pub fn main() {
    assert_eq!(fizz_buzz(1), "1");
    assert_eq!(fizz_buzz(2), "2");
    assert_eq!(fizz_buzz(3), "Fizz");
    assert_eq!(fizz_buzz(4), "4");
    assert_eq!(fizz_buzz(5), "Buzz");
    assert_eq!(fizz_buzz(6), "Fizz");
    assert_eq!(fizz_buzz(7), "7");
    assert_eq!(fizz_buzz(8), "8");
    assert_eq!(fizz_buzz(9), "Fizz");
    assert_eq!(fizz_buzz(10), "Buzz");
    assert_eq!(fizz_buzz(11), "11");
    assert_eq!(fizz_buzz(12), "Fizz");
    assert_eq!(fizz_buzz(13), "13");
    assert_eq!(fizz_buzz(14), "14");
    assert_eq!(fizz_buzz(15), "FizzBuzz");
    println!("All tests passed");
}