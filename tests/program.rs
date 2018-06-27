extern crate brainfuck;

use brainfuck::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[test]
fn test_hello_world() -> Result<(), Box<Error>> {
    let mut file = File::open("hello_world.bf")?;
    let mut code = String::new();
    file.read_to_string(&mut code)?;
    let program = code.into_program();
    assert_eq!(program.deepest_loop(), 1);
    assert_eq!(program.ops().len(), 114);
    Ok(())
}
