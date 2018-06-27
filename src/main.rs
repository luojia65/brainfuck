extern crate brainfuck;

use brainfuck::prelude::*;
use std::error;
use std::io::stdin;
use std::io::stdout;

fn main() -> Result<(), Box<error::Error>> {
    let s = "\
++++++++[->++++++++<]>++++++++.<+++++[->+++++<]>++++.+++++++
..+++.<++++++++[->--------<]>---------------.<+++++++[->++++
+++<]>++++++.<++++[->++++<]>++++++++.+++.------.--------.<++
++++++[->--------<]>---.<++++[->----<]>----.---.<
    ";
    let mut process = Process::new(s, 1048576, stdin(), stdout());
    process.execute()?;
    Ok(())
}
