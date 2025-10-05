use std::collections::{VecDeque};

pub struct LimitedQueue<T> {
    data: VecDeque<T>,
    pub max_size: usize,
}
impl<T> LimitedQueue<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    pub fn enqueue(&mut self, item: T) -> Result<(), &'static str> {
        if self.data.len() >= self.max_size {
            return Err("Queue overflow");
        }
        self.data.push_back(item);
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.data.pop_front()
    }
    
    pub fn peek(&self) -> Option<&T> {
        self.data.back()
    }
    
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

pub struct LimitedStack<T> {
    data: Vec<T>,
    max_size: usize,
}

impl<T> LimitedStack<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            data: Vec::with_capacity(max_size),
            max_size,
        }
    }
    
    pub fn push(&mut self, item: T) -> Result<(), &'static str> {
        if self.data.len() >= self.max_size {
            return Err("Stack overflow");
        }
        self.data.push(item);
        Ok(())
    }
    
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
    
    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }
    
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}