pub fn factorial(n: u32) -> u128 {
    let mut product: u128 = 1;
    let mut i: u128 = n as u128;
    while i > 0 {
        product *= i;
        i -= 1;
    }
    product
}


pub fn main() {
    let result = factorial(5);
    assert_eq!(result, 120);
    
    let result = factorial(10);
    assert_eq!(result, 3628800);
    
    let result = factorial(0);
    assert_eq!(result, 1);
    println!("All tests passed");
}