mod brainfuck;

use std::io::stdin;
use std::io::stdout;

fn main() -> Result<(), brainfuck::Error>{
    let s = ",>++++++[<-------->-],,[<+>-],<.>.";
    let program = brainfuck::Program::from(String::from(s));
    let mut process = brainfuck::Process::new(program, 1024, stdin(), stdout());
    process.execute()
}
