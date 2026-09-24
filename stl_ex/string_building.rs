use std::fmt::{self, Display, Formatter, Write};

/// Build a greeting message using format!
pub fn build_greeting(name: &str, age: u32) -> String {
    // TODO: Use format!() to create a greeting
    // Format: "Hello, {name}! You are {age} years old."
    format!("Hello, {name}! You are {age} years old.")
}

/// Build a numbered list from items using write!
pub fn build_list(items: &[&str]) -> String {
    // TODO: Create a numbered list using std::fmt::Write
    // Format: "1. item1\n2. item2\n3. item3"
    let mut result = String::new();
    for (index, item) in items.iter().enumerate() {
        if index > 0 {
            result.push('\n');
        }
        write!(result, "{}. {}", index + 1, item).unwrap();
    }
    result
}

/// A person with a name and age.
#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

// TODO: Implement Display for Person
// Format: "Name (Age years old)"
impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // TODO: Write the person's info in the format
        write!(f, "{} ({} years old)", self.name, self.age)
    }
}

/// Build a simple text table with headers and data rows.
///
/// The table format uses pipes and dashes:
/// ```text
/// | Name  | Age |
/// |-------|-----|
/// | Alice | 30  |
/// | Bob   | 25  |
/// ```
pub fn build_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    if headers.is_empty() {
        return String::new();
    }
    let mut widths: Vec<usize> = headers.iter().map(|header| header.len()).collect();
    for row in rows {
        for (index, cell) in row.iter().enumerate() {
            if let Some(width) = widths.get_mut(index) {
                *width = (*width).max(cell.len());
            }
        }
    }
    let mut lines = Vec::new();
    let mut header_line = String::new();
    for (header, width) in headers.iter().zip(&widths) {
        write!(header_line, "| {header:<width$} ").unwrap();
    }
    header_line.push('|');
    lines.push(header_line);
    let mut separator = String::new();
    for width in &widths {
        separator.push('|');
        separator.push_str(&"-".repeat(width + 2));
    }
    separator.push('|');
    lines.push(separator);
    for row in rows {
        let mut line = String::new();
        for (index, width) in widths.iter().enumerate() {
            let cell = row.get(index).map(String::as_str).unwrap_or("");
            write!(line, "| {cell:<width$} ").unwrap();
        }
        line.push('|');
        lines.push(line);
    }
    lines.join("\n")
}

/// Concatenate strings with a separator without using .join()
pub fn concat_with_separator(parts: &[&str], sep: &str) -> String {
    // TODO: Join the parts with the separator
    // Don't use the .join() method - implement it manually
    let mut result = String::new();
    for (index, part) in parts.iter().enumerate() {
        result.push_str(part);
        if index < parts.len() - 1 {
            result.push_str(sep);
        }
    }
    result
}

pub fn main() {
    // Demonstrate build_greeting
    println!("=== build_greeting ===");
    println!("{}", build_greeting("Alice", 30));

    // Demonstrate build_list
    println!("\n=== build_list ===");
    println!("{}", build_list(&["apple", "banana", "cherry"]));

    // Demonstrate Person Display
    println!("\n=== Person Display ===");
    let person = Person {
        name: "Bob".to_string(),
        age: 25,
    };
    println!("{}", person);

    // Demonstrate build_table
    println!("\n=== build_table ===");
    let headers = &["Name", "Age"];
    let rows = vec![
        vec!["Alice".to_string(), "30".to_string()],
        vec!["Bob".to_string(), "25".to_string()],
    ];
    println!("{}", build_table(headers, &rows));

    // Demonstrate concat_with_separator
    println!("\n=== concat_with_separator ===");
    println!("{}", concat_with_separator(&["a", "b", "c"], ", "));
}
