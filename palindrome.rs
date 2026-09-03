pub fn find_first_palindrome(start: i32, end: i32) -> Option<i32> {
    let begin = start.min(end);
    let fin = end.max(start);

    for i in begin..=fin {
        if i < 0 {
            continue;
        }
        let s = i.to_string();
        let reversed = s.chars().rev().collect::<String>();
        if s == reversed {
            return Some(i);
        }
    }
    None
}

pub fn main() {
    let result = find_first_palindrome(10, 30);
    // 11 is the first palindrome in the range
    assert_eq!(result, Some(11));
    
    let result = find_first_palindrome(100, 105);
    // 101 is the first palindrome in the range
    assert_eq!(result, Some(101));
    
    let result = find_first_palindrome(123, 130);
    assert_eq!(result, None); // No palindromes in this range
    
    let result = find_first_palindrome(-130, -1);
    assert_eq!(result, None); // No palindromes in this range
    
    let result = find_first_palindrome(100, -105);
    // 0 is the first palindrome in the range
    assert_eq!(result, Some(0));
    println!("All tests passed");
}