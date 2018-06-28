//! An experimental Brainfuck compiler & runtime, built to learn API designing style in Rust.
//!
//! Brainfuck is a minimized programming language. All its programs may be consisted of up to 8
//! operators: `>`, `<`, `+`, `-`, `.`, `,`, `[` and `]`. 

#![feature(crate_visibility_modifier)]

mod error;
mod op;
mod process;
mod program;

pub use error::*;
pub use op::*;
pub use process::*;
pub use program::*;
