#[derive(Debug)]
pub enum Operator {
    PtrPlusOne,
    PtrMinusOne,
    ValuePlusOne,
    ValueMinusOne,
    Print,
    Read,
    LoopBegin,
    LoopEnd,
}
