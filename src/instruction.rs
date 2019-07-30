use crate::value::Value;

#[derive(Debug, Clone)]
pub enum Instr<'a> {
    Jump(usize),
    JumpIfFalse(usize),
    Store(&'a str),
    Load(&'a str),
    BuildList(usize),
    Binary(Binary),
    LoadConst(Value<'a>),
    Print,
}

#[derive(Debug, Clone)]
pub enum Binary {
    Add,
    Sub,
    Mult,
    Div,
}
