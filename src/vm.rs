use std::collections::HashMap;

use crate::instruction::Binary;
use crate::instruction::Instr;
use crate::stack::Stack;
use crate::value::Value;

type VmEnv<'a> = HashMap<&'a str, Value<'a>>;
type VmResult<'a, T> = Result<T, VmError<'a>>;

#[derive(Debug)]
enum VmError<'a> {
    NotInScope(&'a str),
    MismatchedType(&'a str),
    EmptyStack,
}

pub struct Vm<'a> {
    env: VmEnv<'a>,
    stack: Stack<Value<'a>>,
    pointer: usize,
}

impl<'a> Vm<'a> {
    pub fn new() -> Self {
        Vm {
            env: HashMap::new(),
            stack: Stack::new(Vec::new()),
            pointer: 0,
        }
    }

    pub fn run(&mut self, instrs: Vec<Instr<'a>>) -> VmResult<'a, ()> {
        while self.pointer < instrs.len() {
            let instr = instrs[self.pointer];
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
            _ => unreachable!(),
        }
    }

    fn store(&mut self, id: &'a str) -> VmResult<'a, ()> {
        let val = self.stack.pop().ok_or(VmError::EmptyStack)?;
        self.env.insert(id, val);
        Ok(())
    }

    fn load(&mut self, id: &'a str) -> VmResult<'a, ()> {
        let val = self.env.get(id).ok_or(VmError::NotInScope(id))?;
        self.stack.push(val.clone());
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
