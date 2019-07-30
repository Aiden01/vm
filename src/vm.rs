use crate::instruction::Binary;
use crate::instruction::Instr;
use crate::stack::{Frame, Stack};
use crate::value::Value;

type VmResult<'a, T> = Result<T, VmError<'a>>;

#[derive(Debug)]
pub enum VmError<'a> {
    NotInScope(&'a str),
    MismatchedType(&'a str),
    EmptyStack,
}

pub struct Vm<'a> {
    stack: Stack<'a, Value<'a>>,
    pointer: usize,
}

impl<'a> Vm<'a> {
    pub fn new() -> Self {
        Vm {
            stack: Stack::new(Vec::new()),
            pointer: 0,
        }
    }

    pub fn run(&mut self, instrs: Vec<Instr<'a>>) -> VmResult<'a, ()> {
        let frame = Frame::new(instrs.len());
        self.stack.frames.push(frame);
        while self.pointer < instrs.len() {
            let instr = instrs[self.pointer].clone();
            self.pointer += 1;
            self.run_instr(instr)?;
        }

        Ok(())
    }

    fn run_instr(&mut self, instr: Instr<'a>) -> VmResult<'a, ()> {
        match instr {
            Instr::Jump(x) => self.set_pointer(x),
            Instr::Store(id) => self.store(id),
            Instr::Load(id) => self.load(id),
            Instr::BuildList(n) => self.build_list(n),
            Instr::Binary(op) => self.binary_op(op),
            Instr::JumpIfFalse(x) => self.jump_if_false(x),
            Instr::LoadConst(val) => {
                self.stack.push(val);
                Ok(())
            }
            Instr::Print => self.print(),
        }
    }

    fn print(&mut self) -> VmResult<'a, ()> {
        let val = self.stack.pop().ok_or(VmError::EmptyStack)?;
        println!("{:?}", val);
        Ok(())
    }

    fn store(&mut self, id: &'a str) -> VmResult<'a, ()> {
        let val = self.stack.pop().ok_or(VmError::EmptyStack)?;
        self.stack.store_local(id, val);
        Ok(())
    }

    fn load(&mut self, id: &'a str) -> VmResult<'a, ()> {
        let val = self.stack.get_local(id).ok_or(VmError::NotInScope(id))?;
        self.stack.push(val);
        Ok(())
    }

    fn build_list(&mut self, n: usize) -> VmResult<'a, ()> {
        let elems = self.stack.pop_n(n);
        self.stack.push(Value::List(elems));
        Ok(())
    }

    fn set_pointer(&mut self, x: usize) -> VmResult<'a, ()> {
        self.pointer = x;
        Ok(())
    }

    fn ret(&mut self) -> VmResult<'a, ()> {
        let ptr = self.stack.ret();
        self.pointer = ptr;
        Ok(())
    }

    fn jump_if_false(&mut self, to: usize) -> VmResult<'a, ()> {
        let val = self.stack.pop().ok_or(VmError::EmptyStack)?;
        match val {
            Value::Bool(false) => self.set_pointer(to),
            Value::Bool(_) => Ok(()),
            _ => Err(VmError::MismatchedType("boolean")),
        }
    }

    fn binary_op(&mut self, op: Binary) -> VmResult<'a, ()> {
        let values = self.stack.pop2().ok_or(VmError::EmptyStack)?;
        let result = match op {
            Binary::Add => self.add(values),
            Binary::Sub => self.sub(values),
            Binary::Mult => self.mult(values),
            Binary::Div => self.div(values),
        }?;

        self.stack.push(result);
        Ok(())
    }

    fn add(&mut self, values: (Value<'a>, Value<'a>)) -> VmResult<'a, Value<'a>> {
        match values {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Int(a), Value::Float(b)) | (Value::Float(b), Value::Int(a)) => {
                Ok(Value::Float((a as f64) + b))
            }
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            _ => Err(VmError::MismatchedType("TODO")),
        }
    }

    fn sub(&mut self, values: (Value<'a>, Value<'a>)) -> VmResult<'a, Value<'a>> {
        match values {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Int(a), Value::Float(b)) | (Value::Float(b), Value::Int(a)) => {
                Ok(Value::Float((a as f64) - b))
            }
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            _ => Err(VmError::MismatchedType("TODO")),
        }
    }

    fn div(&mut self, values: (Value<'a>, Value<'a>)) -> VmResult<'a, Value<'a>> {
        match values {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Int(a), Value::Float(b)) | (Value::Float(b), Value::Int(a)) => {
                Ok(Value::Float((a as f64) * b))
            }
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            _ => Err(VmError::MismatchedType("TODO")),
        }
    }

    fn mult(&mut self, values: (Value<'a>, Value<'a>)) -> VmResult<'a, Value<'a>> {
        match values {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a / b)),
            (Value::Int(a), Value::Float(b)) | (Value::Float(b), Value::Int(a)) => {
                Ok(Value::Float((a as f64) / b))
            }
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
            _ => Err(VmError::MismatchedType("TODO")),
        }
    }
}
