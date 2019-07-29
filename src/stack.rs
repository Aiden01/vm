pub struct Stack<T> {
    data: Vec<T>,
}

impl<T: Clone> Stack<T> {
    pub fn new(data: Vec<T>) -> Self {
        Stack { data }
    }

    pub fn pop_n(&mut self, n: usize) -> Vec<T> {
        self.data.split_off(n)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.iter().cloned().nth(0)
    }

    pub fn pop2(&mut self) -> Option<(T, T)> {
        let first = self.pop()?;
        let last = self.pop()?;
        Some((first, last))
    }

    pub fn push2(&mut self, values: (T, T)) {
        let (a, b) = values;
        self.push(a);
        self.push(b);
    }

    pub fn push(&mut self, x: T) {
        self.data.insert(0, x);
    }
}
