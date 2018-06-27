/// All 8 operators which brainfuck programming language is based on.
///
/// A brainfuck program runs with two pointers: *memory pointer*("`ptr`") and *operation pointer*.
#[derive(Debug)]
pub enum Operator {
    /// Operator `>` that adds the memory pointer by 1, which is like `ptr = ptr + 1`.
    PtrPlusOne,
    /// Operator `<` that subs the memory pointer by 1, which is like `ptr = ptr - 1`.
    PtrMinusOne,
    /// Operator `+` that adds the memory data that is pointed by 1, which is like `*ptr = *ptr + 1`.
    ValuePlusOne,
    /// Operator `-` that subs the memory data that is pointed by 1, which is like `*ptr = *ptr - 1`.
    ValueMinusOne,
    /// Operator `.` that prints a `u8` as `char` to Write with the memory data that is pointed,
    /// which is like `print!("{}", *ptr as char)`.
    Print,
    /// Operator `,` that reads a `char` as `u8` from Read to the the memory data that is pointed,
    /// which is like `cin>>*ptr` in C++.
    Read,
    /// Operator `[` marks the beginning of a loop.
    ///
    /// Matched with `LoopEnd`, `LoopBegin` is like `while *ptr != 0 {` in some languages.
    LoopBegin,
    /// Operator `]` marks the end of a loop. If the memory data that is pointed is equal to 0,
    /// the execution jumps to the matched `[` mark then continues; otherwise, it won't jump but
    /// will continue anyway.
    ///
    /// Matched with `LoopBegin`, `LoopEnd` is like `}`.
    LoopEnd,
}
