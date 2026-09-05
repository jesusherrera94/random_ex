fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

pub fn convert_temperature(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    if from_unit == "C" {
         match to_unit {
             "F" => Ok(round2(value * (9.0/5.0) + 32.0)),
             "K" => Ok(round2(value + 273.15)),
             _ => Err("Invalid unit".to_string())
         }
    } else if from_unit == "F" {
         match to_unit {
             "C" => Ok(round2((value - 32.0) * (5.0/9.0))),
             "K" => Ok(round2((value - 32.0) * (5.0/9.0) + 273.15)),
             _ => Err("Invalid unit".to_string())
         }
    } else if from_unit == "K" {
     match to_unit {
             "C" => Ok(round2(value - 273.15)),
             "F" => Ok(round2((value - 273.15) * (9.0/5.0) + 32.0)),
             _ => Err("Invalid unit".to_string())
         }
    } else {
     Err("Invalid unit".to_string())
    }
 }
 
 
 pub fn main() {
    let result = convert_temperature(100.0, "C", "F");
    assert_eq!(result, Ok(212.0));
    
    let result = convert_temperature(32.0, "F", "C");
    assert_eq!(result, Ok(0.0));
    
    let result = convert_temperature(0.0, "C", "K");
    assert_eq!(result, Ok(273.15));
    
    let result = convert_temperature(300.0, "K", "C");
    assert_eq!(result, Ok(26.85));
    
    let result = convert_temperature(100.0, "C", "X");
    assert_eq!(result, Err("Invalid unit".to_string()));
    println!("All tests passed");
 }