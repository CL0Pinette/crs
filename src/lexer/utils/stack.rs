pub(crate) struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub(crate) fn new() -> Self {
        Stack { stack: Vec::new() }
    }
    pub(crate) fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
    pub(crate) fn push(&mut self, item: T) {
        self.stack.push(item);
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    pub(crate) fn length(&self) -> usize {
        self.stack.len()
    }
    pub(crate) fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}
