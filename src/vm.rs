//! A virtual machine that executes Lox instructions as bytecode.

use crate::chunk::{Chunk, OpCode};
use crate::value::Value;

#[derive(Debug, Default)]
pub struct VM {
    debug: bool,
    stack: Vec<Value>,
}

#[derive(Debug)]
pub enum InterpretError {
    CompileError,
    RuntimeError,
}

impl VM {
    pub fn new(debug: bool) -> Self {
        Self {
            debug,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> Result<(), InterpretError> {
        for op in chunk.iter() {
            match op {
                OpCode::Constant(val) => {
                    self.push(val.as_ref().clone());
                }
                OpCode::Add => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a + b);
                }
                OpCode::Subtract => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a - b);
                }
                OpCode::Multiply => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a * b);
                }
                OpCode::Divide => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a / b);
                }
                OpCode::Negate => {
                    let value = self.pop()?;
                    self.push(-value);
                }
                OpCode::Return => {
                    println!("{}", self.pop()?);
                }
            }
        }

        Ok(())
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);

        if self.debug {
            println!("{:?}", self.stack);
        }
    }

    fn pop(&mut self) -> Result<Value, InterpretError> {
        self.stack.pop().ok_or(InterpretError::CompileError)
    }
}

#[cfg(test)]
mod test {
    use crate::chunk::{constant, OpCode};
    use crate::vm::{InterpretError, VM};

    fn interpret(chunk: Vec<OpCode>) -> Result<VM, InterpretError> {
        let mut vm = VM::default();
        vm.interpret(&chunk.into())?;
        Ok(vm)
    }

    #[test]
    fn test_negate() {
        // Error case
        interpret(vec![OpCode::Negate]).expect_err("Negation needs an argument");

        // Success case
        let mut vm = interpret(vec![constant(1.0), OpCode::Negate]).unwrap();
        assert_eq!(vm.stack.pop().unwrap(), -1.0);
    }

    #[test]
    fn test_add() {
        // Error cases
        interpret(vec![
            OpCode::Add,
        ]).expect_err("Addition needs two arguments");
        interpret(vec![
            constant(1.0),
            OpCode::Add,
        ]).expect_err("Addition needs two arguments");

        // Success case
        let mut vm = interpret(vec![
            constant(1.0),
            constant(2.0),
            OpCode::Add,
        ]).unwrap();
        assert_eq!(vm.stack.pop().unwrap(), 3.0);
    }

    #[test]
    fn test_subtract() {
        // Error cases
        interpret(vec![
            OpCode::Subtract,
        ]).expect_err("Subtract needs two arguments");
        interpret(vec![
            constant(1.0),
            OpCode::Subtract,
        ]).expect_err("Subtract needs two arguments");

        // Success case
        let mut vm = interpret(vec![
            constant(5.0),
            constant(2.0),
            OpCode::Subtract,
        ]).unwrap();
        assert_eq!(vm.stack.pop().unwrap(), 3.0);
    }

    #[test]
    fn test_multiply() {
        // Error cases
        interpret(vec![
            OpCode::Multiply,
        ]).expect_err("Multiply needs two arguments");
        interpret(vec![
            constant(1.0),
            OpCode::Multiply,
        ]).expect_err("Multiply needs two arguments");

        // Success case
        let mut vm = interpret(vec![
            constant(2.0),
            constant(3.0),
            OpCode::Multiply,
        ]).unwrap();
        assert_eq!(vm.stack.pop().unwrap(), 6.0);
    }

    #[test]
    fn test_divide() {
        // Error cases
        interpret(vec![
            OpCode::Divide,
        ]).expect_err("Multiply needs two arguments");
        interpret(vec![
            constant(1.0),
            OpCode::Divide,
        ]).expect_err("Multiply needs two arguments");

        // Success case
        let mut vm = interpret(vec![
            constant(12.0),
            constant(4.0),
            OpCode::Divide,
        ]).unwrap();
        assert_eq!(vm.stack.pop().unwrap(), 3.0);
    }
}