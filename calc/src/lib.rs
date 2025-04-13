// build a calculator library in rust
// with the following functions add, subtract, multiply, divide
// start
// add
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// subtract
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

// multiply
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// divide
pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}
