pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

pub fn main() {
    assert_eq!(fibonacci(0), 0); // Returns 0
    assert_eq!(fibonacci(1), 1); // Returns 1
    assert_eq!(fibonacci(2), 1); // Returns 1 + 0 = 1
    assert_eq!(fibonacci(3), 2); // Returns 1 + 1 = 2
    assert_eq!(fibonacci(4), 3); // Returns 2 + 1 = 3
    assert_eq!(fibonacci(5), 5); // Returns 3 + 2 = 5
    assert_eq!(fibonacci(6), 8); // Returns 5 + 3 = 8
    assert_eq!(fibonacci(7), 13); // Returns 8 + 5 = 13
    println!("All tests passed");
}