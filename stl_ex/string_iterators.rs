/// Convert a string into a vector of its characters.
pub fn chars_to_vec(s: &str) -> Vec<char> {
    // TODO: Use .chars() and .collect()
    s.chars().collect()
}

/// Split a string on whitespace and collect into owned strings.
pub fn words_to_vec(s: &str) -> Vec<String> {
    // TODO: Use .split_whitespace() and collect
    s.split_whitespace()
    .map(|word| word.to_string())
    .collect()
}

/// Split a string on line endings and collect into owned strings.
pub fn lines_to_vec(s: &str) -> Vec<String> {
    // TODO: Use .lines() and collect
    s.lines().collect::<Vec<_>>().into_iter().map(|line| line.to_string())
    .collect::<Vec<_>>()
}

/// Count the number of whitespace-separated words.
pub fn count_words(s: &str) -> usize {
    // TODO: Use .split_whitespace() and .count()
    s.split_whitespace().count()
}

/// Reverse the order of words in a string (keep words themselves intact).
pub fn reverse_words(s: &str) -> String {
    // TODO: Split into words, reverse the order, then join with spaces
    s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}

/// Capitalize the first letter of each word.
pub fn capitalize_words(s: &str) -> String {
    // TODO: Make the first character uppercase and rest lowercase
    s.split_whitespace()
    .map(|word| {
        let mut chars = word.chars();
        chars.next().unwrap().to_uppercase().to_string() + &chars.as_str().to_lowercase()
    })
    .collect::<Vec<_>>()
    .join(" ")
}

pub fn main() {
    let text = "hello world";
    println!("Original: '{}'", text);
    println!("chars_to_vec: {:?}", chars_to_vec(text));
    println!("words_to_vec: {:?}", words_to_vec(text));
    println!("count_words: {}", count_words(text));
    println!("reverse_words: '{}'", reverse_words(text));
    println!("capitalize_words: '{}'", capitalize_words(text));

    let multiline = "line one\nline two\nline three";
    println!("\nMultiline text:");
    println!("{}", multiline);
    println!("lines_to_vec: {:?}", lines_to_vec(multiline));
}
