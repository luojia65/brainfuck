use std::error;
use std::fmt;
use std::io;

/// Errors that could happen in brainfuck compiling and executing.
#[derive(Debug)]
pub enum Error {
    /// Loop End flag `]` appears when there is no enough Loop Begin flag `[` on the left.
    ///
    /// This might happen when the square brackets does not match themselves in a brainfuck
    /// program. For example, `[]]` as there is only one `[` but two `]`'s.
    LoopStackUnderflow,
    /// Represents that the program has already reached the end.
    ///
    /// Sometimes this is not an error - it shows that this program has finished its running.
    ///
    /// This value is only used internally yet. If you're using `match`, consider `_ => {}`
    /// or `unreachable!()`
    PointerOverflow,
    /// Shows that an I/O error had happened when executing this brainfuck program.
    ///
    /// This redirects the error produced by low-level system when executing Input Read flag
    /// `,` and Output Write flag `.`.
    IoError(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}
 