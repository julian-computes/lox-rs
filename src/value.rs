//! This module provides representations of Values,
//! which are simply represented as 64-bit floating-point numbers that Lox
//! users can use to perform computations.

pub type Value = f64;

pub struct ValueArray {
    values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> Self {
        Self {
            values: Vec::new()
        }
    }

    pub fn write(&mut self, value: Value) {
        self.values.push(value);
    }
}