#[macro_export]
macro_rules! math_operations {
  ($a:expr, $op:tt, $b:expr) => {{
    let result = match $op {
        "+" => ($a + $b).to_string(),
        "-" => ($a - $b).to_string(),
        "*" => ($a * $b).to_string(),
        "/" => {
            if $b == 0 {
                "Division by zero".to_string()
            } else {
                ($a / $b).to_string()
            }
            },
        _ => format!("Unsupported operator: {}", $op),
    };
    if result == "Division by zero" || result.contains("Unsupported operator") {
        panic!("{}", result)
    } else {
        format!("{} {} {} = {}", $a, $op, $b, result)
    }
  }};
  }

// Example usage
pub fn main() {
    assert_eq!(math_operations!(4, "+", 2), "4 + 2 = 6");
    assert_eq!(math_operations!(10, "-", 3), "10 - 3 = 7");
    assert_eq!(math_operations!(6, "*", 4), "6 * 4 = 24");
    assert_eq!(math_operations!(15, "/", 3), "15 / 3 = 5");
}
