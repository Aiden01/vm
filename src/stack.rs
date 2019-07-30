use std::collections::HashMap;

type Env<'a, T> = HashMap<&'a str, T>;

#[derive(Debug, Clone)]
pub struct Frame<'a, T> {
    pub ret_addr: usize,
    pub locals: Env<'a, T>,
    pub data: Vec<T>
}

impl<'a, T: Clone> Frame<'a, T> {
    pub fn new(ret_addr: usize) -> Self {
        Frame {
            ret_addr,
            locals: HashMap::new(),
            data: Vec::new()
        }
    }
}

pub struct Stack<'a, T> {
    pub frames: Vec<Frame<'a, T>>
}

impl<'a, T: Clone> Stack<'a, T> {

    pub fn new(frames: Vec<Frame<'a, T>>) -> Self {
        Stack { frames }
    }

    fn current_frame(&mut self) -> &mut Frame<'a, T> {
        self.frames.last_mut()
            .expect("Call stack is empty")
    }

    pub fn push(&mut self, data: T) {
        let frame = self.current_frame();
        frame.data.push(data);
    }

    pub fn push2(&mut self, data: (T, T)) {
        let (a, b) = data;
        self.push(a);
        self.push(b);
    }

    pub fn pop_n(&mut self, n: usize) -> Vec<T> {
        let frame = self.current_frame();
        frame.data.split_off(frame.data.len() + 1 - n)
    }

    pub fn pop(&mut self) -> Option<T> {
        let frame = self.current_frame();
        frame.data.pop()
    }

    pub fn pop2(&mut self) -> Option<(T, T)> {
        let a = self.pop()?;
        let b = self.pop()?;
        Some((a, b))
    }

    pub fn store_local(&mut self, id: &'a str, val: T) {
        let frame = self.current_frame();
        frame.locals.insert(id, val);
    }

    pub fn get_local(&mut self, id: &'a str) -> Option<T> {
        let frame = self.current_frame();
        frame.locals.get(id).cloned()
    }

    pub fn ret(&mut self) -> usize {
        let frame = self.frames.pop().expect("Call stack is empty");
        frame.ret_addr
    }
    
}
