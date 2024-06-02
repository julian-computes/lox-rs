//! “Yes, they actually wrote machine code by hand. On punched cards. Which,
//! presumably, they punched with their fists.”
//! Excerpt From
//! Crafting Interpreter
//! Robert Nystrom

use std::rc::Rc;
use crate::chunk::{Chunk, OpCode};
use crate::vm::{VM};

mod chunk;
mod value;
mod vm;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::Constant(Rc::new(24.234343)));
    chunk.write(OpCode::Negate);
    chunk.write(OpCode::Return);
    println!("{}", chunk);

    let mut vm = VM::new(true);
    match vm.interpret(&chunk) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error: {err:?}")
        }
    }
}
