use op::Operator;
use std::rc::Rc;

/// A brainfuck program that can be executed after created into a [`Process`].
///
/// [`Process`]: struct.Process.html
#[derive(Clone)]
pub struct Program {
    ops: Rc<Vec<Operator>>,
    deepest_loop: usize,
}

impl Program {
    /// All operators in this program that could be executed later in a [`Process`].
    ///
    /// As brainfuck programs could include redirect operators like `[` and `]`, the return value
    /// of this function should provide a random access. Iterators are not allowed.
    ///
    /// [`Process`]: struct.Process.html
    pub fn ops(&self) -> Rc<Vec<Operator>> {
        Rc::clone(&self.ops)
    }

    /// Provides maximum loop depth, which is useful when creating a process as the size of loop
    /// stack is given by this function.
    pub fn deepest_loop(&self) -> usize {
        self.deepest_loop
    }
}

/// What can be compiled into a program. That could be a `&str` or a `String`.
pub trait IntoProgram {
    fn into_program(self) -> Program;
}

impl<'a> IntoProgram for &'a str {
    fn into_program(self) -> Program {
        String::from(self).into_program()
    }
}

impl IntoProgram for String {
    fn into_program(self) -> Program {
        let mut ops = Vec::with_capacity(self.len());
        let mut deepest_loop = 0;
        let mut loop_depth = 0;
        for ch in self.chars() {
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
                }
                ']' => {
                    loop_depth = loop_depth - 1;
                    Operator::LoopEnd
                }
                _ => continue
            };
            ops.push(op);
        }
        Program {
            ops: Rc::from(ops),
            deepest_loop,
        }
    }
}

