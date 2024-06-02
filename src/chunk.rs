//! A `chunk` is a sequence of bytecode.

use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::slice::Iter;
use crate::chunk::OpCode::Constant;

use crate::value::Value;

/// An instruction identifier.
#[derive(Debug)]
#[repr(u8)]
pub enum OpCode {
    /// Produce a constant
    Constant(Rc<Value>),
    /// Binary addition
    Add,
    /// Binary subtraction
    Subtract,
    /// Binary multiplication
    Multiply,
    /// Binary division
    Divide,
    /// Unary negation
    Negate,
    /// Return from the current function
    Return,
}

pub fn constant(val: Value) -> OpCode {
    Constant(Rc::new(val))
}

/// A sequence of `OpCode`s.
#[derive(Debug)]
pub struct Chunk {
    code: Vec<OpCode>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
        }
    }

    pub fn iter(&self) -> Iter<'_, OpCode> {
        self.code.iter()
    }

    pub fn write(&mut self, op_code: OpCode) {
        self.code.push(op_code);
    }
}

impl From<Vec<OpCode>> for Chunk {
    fn from(value: Vec<OpCode>) -> Self {
        let mut chunk = Chunk::new();
        for val in value {
            chunk.write(val);
        }
        chunk
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "== disassembled ==\n")?;

        for (line, op) in self.code.iter().enumerate() {
            write!(f, "{line: <4}  {op:?}\n")?;
        }

        Ok(())
    }
}