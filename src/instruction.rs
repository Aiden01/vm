#[derive(Debug, Copy, Clone)]
pub enum Instr<'a> {
    Jump(usize),
    JumpIfFalse(usize),
    Store(&'a str),
    Load(&'a str),
    BuildList(usize)
}
