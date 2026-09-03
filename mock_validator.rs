pub fn validate_user(age: i32, email: &str) -> Result<(), String> {
    if age < 0 || age > 120 {
        return Err("Invalid age".to_string());
    } if !email.contains("@") {
        return Err("Invalid email".to_string());
    }
    Ok(())
}

pub fn main() {
    let result = validate_user(20, "test@example.com");
    assert_eq!(result, Ok(()));
    
    let result = validate_user(-1, "test@example.com");
    assert_eq!(result, Err("Invalid age".to_string()));
    
    let result = validate_user(20, "testexample.com");
    assert_eq!(result, Err("Invalid email".to_string()));
    println!("All tests passed");
}