/// Remove leading and trailing whitespace, then convert to lowercase.
pub fn clean_string(s: &str) -> String {
    let text = s.trim_start();
    text.trim_end().to_lowercase().to_string()
}

/// Check if the text contains the given word (case-insensitive).
pub fn contains_word(text: &str, word: &str) -> bool {
    text.to_lowercase().contains(&word.to_lowercase())
}

/// Replace all occurrences of `from` with `to`.
pub fn replace_word(text: &str, from: &str, to: &str) -> String {
    text.replace(from, to)
}

/// Split the string by the delimiter and trim each part.
pub fn split_and_trim(s: &str, delimiter: char) -> Vec<String> {
    s.split(delimiter)
        .map(|part| part.trim().to_string())
        .collect()
}

/// Replace all sequences of whitespace with a single space, and trim.
pub fn normalize_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<&str>>().join(" ")
}

pub fn main() {
    let messy = "  Hello, World!  ";
    println!("Original: '{}'", messy);
    println!("Cleaned: '{}'", clean_string(messy));

    let text = "Rust is a systems programming language";
    println!("\nText: '{}'", text);
    println!("Contains 'SYSTEMS': {}", contains_word(text, "SYSTEMS"));
    println!("Contains 'Java': {}", contains_word(text, "Java"));

    let original = "hello world world";
    println!("\nOriginal: '{}'", original);
    println!("Replace 'world' with 'Rust': '{}'", replace_word(original, "world", "Rust"));

    let csv = "  apple ,  banana  , cherry ";
    println!("\nCSV: '{}'", csv);
    println!("Split and trim: {:?}", split_and_trim(csv, ','));

    let spaced = "  too   many    spaces   here  ";
    println!("\nSpaced: '{}'", spaced);
    println!("Normalized: '{}'", normalize_whitespace(spaced));
    assert_eq!(
        normalize_whitespace("  hello    world  "),
        "hello world"
    );
    println!("All tests passed");
}
