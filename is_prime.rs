pub fn is_prime(n: u32) -> bool {
    let mut accum: u32 = 0;
    for i in 1..=n {
        if n % i == 0 {
            accum += 1;
        }
    }
    accum == 2
}

pub fn main() {
    assert_eq!(is_prime(1), false);
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(4), false);
    assert_eq!(is_prime(5), true);
    assert_eq!(is_prime(6), false);
    assert_eq!(is_prime(7), true);
    assert_eq!(is_prime(8), false);
    assert_eq!(is_prime(9), false);
    assert_eq!(is_prime(10), false);
    println!("All tests passed");
}