use std::collections::HashMap;

use crate::value::Value;
use crate::stack::Stack;
use crate::instruction::Instr;

type VmEnv<'a> = HashMap<&'a str, Value<'a>>;
type VmResult<'a, T> = Result<T, VmError<'a>>;

#[derive(Debug)]
enum VmError<'a> {
    NotInScope(&'a str),
    MismatchedType(&'a str),
    EmptyStack
}

pub struct Vm<'a> {
    env: VmEnv<'a>,
    stack: Stack<Value<'a>>,
    pointer: usize
}

impl<'a> Vm<'a> {
    
    pub fn new() -> Self {
        Vm {
            env: HashMap::new(),
            stack: Stack::new(Vec::new()),
            pointer: 0
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
            _ => unreachable!()
        };

        Ok(())
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

    fn set_pointer(&mut self, x: usize) -> VmResult<'a, ()> {
        self.pointer = x;
        Ok(())
    }

    fn jump_if_false(&mut self, to: usize) -> VmResult<'a, ()> {
        let val = self.stack.pop().ok_or(VmError::EmptyStack)?;
        match val {
            Value::Bool(false) => self.set_pointer(to),
            Value::Bool(_) => Ok(()),
            _ => Err(VmError::MismatchedType("boolean"))
        }
    }
    
}
