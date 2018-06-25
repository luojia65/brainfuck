extern crate std;

use std::fmt::Formatter;
use std::io::{Read, Write};

pub struct Process {
    ops: Vec<Operator>,
    op_ptr: usize,
    memory: Vec<u8>,
    memory_ptr: usize,
    loop_ptr_stack: Vec<usize>,
    input: Box<Read>,
    output: Box<Write>,
}

#[derive(Debug)]
pub enum Operator {
    PtrPlusOne,
    PtrMinusOne,
    ValuePlusOne,
    ValueMinusOne,
    Print,
    Read,
    LoopBegin,
    LoopEnd
}

pub struct Program {
    ops: Vec<Operator>,
    deepest_loop: usize
}

impl From<String> for Program {
    fn from(s: String) -> Self {
        let mut ops = Vec::new();
        let mut deepest_loop = 0;
        let mut loop_depth = 0;
        for ch in s.chars() {
            let op = match ch {
                '>' => Operator::PtrPlusOne,
                '<' => Operator::PtrMinusOne,
                '+' => Operator::ValuePlusOne,
                '-' => Operator::ValueMinusOne,
                '.' => Operator::Print,
                ',' => Operator::Read,
                '[' => {
                    loop_depth = loop_depth + 1;
                    if loop_depth > deepest_loop {
                        deepest_loop = loop_depth;
                    }
                    Operator::LoopBegin
                },
                ']' => {
                    loop_depth = loop_depth - 1;
                    Operator::LoopEnd
                },
                _ => continue
            };
            ops.push(op);
        }
        Program {
            ops,
            deepest_loop
        }
    }
}

#[derive(Debug)]
pub enum Error {
    LoopStackUnderflow,
    PointerOverflow,
    IoError(std::io::Error)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IoError(err)
    }
}

impl Process {

    pub fn new<I, O>(program: Program, mem_size: usize, input: I, output: O) -> Process
        where I: Read + 'static, O: Write + 'static {
        let mut memory = Vec::with_capacity(mem_size);
        for _ in 0..mem_size {
            memory.push(0);
        }
        Process {
            ops: program.ops,
            op_ptr: 0,
            memory,
            memory_ptr: 0,
            loop_ptr_stack: Vec::with_capacity(program.deepest_loop),
            input: Box::new(input),
            output: Box::new(output),
        }
    }

    pub fn execute(&mut self) -> Result<(), Error> {
        loop {
            match self.next_step() {
                Ok(_) => continue,
                Err(Error::PointerOverflow) => return Ok(()),
                Err(e) => return Err(e),
            };
        }
    }

    pub fn next_step(&mut self) -> Result<(), Error> {
        if self.op_ptr == self.ops.len() {
            return Err(Error::PointerOverflow)
        }
        let op = &self.ops[self.op_ptr];
        let next_ptr = match *op {
            Operator::PtrPlusOne => {
                self.memory_ptr = self.memory_ptr + 1;
                self.op_ptr + 1
            },
            Operator::PtrMinusOne => {
                self.memory_ptr = self.memory_ptr - 1;
                self.op_ptr + 1
            },
            Operator::ValuePlusOne => {
                self.memory[self.memory_ptr] = self.memory[self.memory_ptr] + 1;
                self.op_ptr + 1
            },
            Operator::ValueMinusOne => {
                self.memory[self.memory_ptr] = self.memory[self.memory_ptr] - 1;
                self.op_ptr + 1
            },
            Operator::Print => {
                self.output.write(&vec![self.memory[self.memory_ptr]])?;
                self.op_ptr + 1
            },
            Operator::Read => {
                let mut buf = [0u8; 1];
                self.input.read_exact(&mut buf)?;
                self.memory[self.memory_ptr] = buf[0];
                self.op_ptr + 1
            },
            Operator::LoopBegin => {
                self.loop_ptr_stack.push(self.op_ptr);
                self.op_ptr + 1
            },
            Operator::LoopEnd => {
                match self.loop_ptr_stack.pop() {
                    Some(p) => {
                        if self.memory[self.memory_ptr] == 0 {
                            self.op_ptr + 1
                        } else {
                            p
                        }
                    },
                    None => return Err(Error::LoopStackUnderflow)
                }
            },
        };
        self.op_ptr = next_ptr;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use std::io::stdin;
    use std::io::stdout;
    use brainfuck::*;

    #[test]
    fn test_hello_world() -> Result<(), Box<std::error::Error>> {
        let mut file = File::open("hello_world.bf")?;
        let mut code = String::new();
        file.read_to_string(&mut code)?;
        let program = Program::from(code);
        assert_eq!(program.deepest_loop, 1);
        assert_eq!(program.ops.len(), 114);
        let mut process = Process::new(program, 1024, stdin(), stdout());
        assert_eq!(process.memory.capacity(), 1024);
        process.execute()?;
        Ok(())
    }
}