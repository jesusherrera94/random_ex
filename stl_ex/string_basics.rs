/// Convert a string slice to an owned String.
pub fn to_owned_string(s: &str) -> String {
    s.to_string()
}

/// Count the number of Unicode characters in a string.
pub fn count_chars(s: &str) -> usize {
    s.chars().count()
}

/// Count the number of bytes in a string.
pub fn count_bytes(s: &str) -> usize {
    s.bytes().count()
}

/// Check if a string contains only ASCII characters.
pub fn is_ascii_only(s: &str) -> bool {
    s.is_ascii()
}

/// Return the first character of a string, or None if the string is empty.
pub fn first_char(s: &str) -> Option<char> {
    if !s.is_empty() {
        Some(s.chars().next().unwrap())
    } else {
        None
    }    
}

pub fn main() {
    let greeting = "Hello, world!";

    println!("Original: {}", greeting);
    println!("As owned String: {}", to_owned_string(greeting));
    println!("Character count: {}", count_chars(greeting));
    println!("Byte count: {}", count_bytes(greeting));
    println!("Is ASCII only: {}", is_ascii_only(greeting));
    println!("First character: {:?}", first_char(greeting));
    assert_eq!(
        to_owned_string("hello"),
        String::from("hello")
    );
    assert_eq!(count_chars("hello"), 5);
    // 5 Unicode characters (cafe + combining accent)
    assert_eq!(count_chars("cafe\u{0301}"), 5);
    assert_eq!(count_bytes("hello"), 5);
    // 5 bytes: 'c', 'a', 'f' = 1 byte each
    // 'e with accent' = 2 bytes
    assert_eq!(count_bytes("caf\u{00E9}"), 5);
    assert_eq!(is_ascii_only("hello"), true);
    // accented 'e' is not ASCII
    assert_eq!(is_ascii_only("caf\u{00E9}"), false);
    assert_eq!(first_char("hello"), Some('h'));
    assert_eq!(first_char(""), None);
    println!("All tests passed");
}
