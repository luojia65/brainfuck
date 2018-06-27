use error::Error;
use op::Operator;
use program::IntoProgram;
use std::io::{Read, Write};
use std::rc::Rc;

/// A brainfuck process, containing the program as operation vector, the memory, input, output
/// and so on.
///
/// This process may be created using [`Process::new`].
///
/// [`Process::new`]: struct.Process.html#new
pub struct Process {
    ops: Rc<Vec<Operator>>,
    op_ptr: usize,
    memory: Vec<u8>,
    memory_ptr: usize,
    loop_ptr_stack: Vec<usize>,
    input: Box<Read>,
    output: Box<Write>,
}

impl Process {
    /// Creates a process from given brainfuck program.
    ///
    /// This function does not actually check the compile errors such as unmatched `[` or `]`,
    /// pointer underflow, pointer overflow, out of memory and so on.
    /// Therefore, this compiler is not safe enough as rustc :)
    ///
    /// After using `Process::new`, you may execute using function [`execute()`].
    ///
    /// [`execute()`]: struct.Process.html#execute
    pub fn new<P, I, O>(program: P, mem_size: usize, input: I, output: O) -> Process
        where P: IntoProgram, I: Read + 'static, O: Write + 'static {
        let mut memory = Vec::with_capacity(mem_size);
        for _ in 0..mem_size {
            memory.push(0);
        }
        let program = program.into_program();
        Process {
            ops: program.ops(),
            op_ptr: 0,
            memory,
            memory_ptr: 0,
            loop_ptr_stack: Vec::with_capacity(program.deepest_loop()),
            input: Box::new(input),
            output: Box::new(output),
        }
    }

    /// Executes this brainfuck process containing the program, the allocated memory, and the
    /// input and output devices defined in function `new`.
    ///
    /// Instead of by parameters and return values, the input and output is transferred by input
    /// and output devices.
    ///
    /// # Errors
    ///
    /// Errors occurred in execution. See [`brainfuck::Error`].
    ///
    /// [`brainfuck::Error`]: enum.Error.html
    #[inline]
    pub fn execute(&mut self) -> Result<(), Error> {
        loop {
            match self.next_step() {
                Ok(_) => continue,
                Err(Error::PointerOverflow) => return Ok(()),
                Err(e) => return Err(e),
            };
        }
    }

    fn next_step(&mut self) -> Result<(), Error> {
        if self.op_ptr == self.ops.len() {
            return Err(Error::PointerOverflow);
        }
        let op = &self.ops[self.op_ptr];
        let next_ptr = match *op {
            Operator::PtrPlusOne => {
                self.memory_ptr = self.memory_ptr + 1;
                self.op_ptr + 1
            }
            Operator::PtrMinusOne => {
                self.memory_ptr = self.memory_ptr - 1;
                self.op_ptr + 1
            }
            Operator::ValuePlusOne => {
                if self.memory[self.memory_ptr] == <u8>::max_value() {
                    self.memory[self.memory_ptr] = 0;
                } else {
                    self.memory[self.memory_ptr] = self.memory[self.memory_ptr] + 1;
                }
                self.op_ptr + 1
            }
            Operator::ValueMinusOne => {
                if self.memory[self.memory_ptr] == 0 {
                    self.memory[self.memory_ptr] = <u8>::max_value();
                } else {
                    self.memory[self.memory_ptr] = self.memory[self.memory_ptr] - 1;
                }
                self.op_ptr + 1
            }
            Operator::Print => {
                self.output.write(&vec![self.memory[self.memory_ptr]])?;
                self.op_ptr + 1
            }
            Operator::Read => {
                let mut buf = [0u8; 1];
                self.input.read_exact(&mut buf)?;
                self.memory[self.memory_ptr] = buf[0];
                self.op_ptr + 1
            }
            Operator::LoopBegin => {
                self.loop_ptr_stack.push(self.op_ptr);
                self.op_ptr + 1
            }
            Operator::LoopEnd => {
                match self.loop_ptr_stack.pop() {
                    Some(p) => {
                        if self.memory[self.memory_ptr] == 0 {
                            self.op_ptr + 1
                        } else {
                            p
                        }
                    }
                    None => return Err(Error::LoopStackUnderflow)
                }
            }
        };
        self.op_ptr = next_ptr;
        Ok(())
    }
}

