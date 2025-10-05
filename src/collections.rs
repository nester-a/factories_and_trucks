use std::collections::VecDeque;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_queue() {
        let queue: LimitedQueue<i8> = LimitedQueue::new(1);
        assert_eq!(queue.max_size, 1);
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_enqueue_and_len() {
        let mut queue: LimitedQueue<i8> = LimitedQueue::new(3);
        
        assert!(queue.enqueue(1).is_ok());
        assert_eq!(queue.len(), 1);
        
        assert!(queue.enqueue(2).is_ok());
        assert_eq!(queue.len(), 2);
        
        assert!(queue.enqueue(3).is_ok());
        assert_eq!(queue.len(), 3);
    }

    #[test]
    fn test_enqueue_overflow() {
        let mut queue: LimitedQueue<i8> = LimitedQueue::new(2);
        
        assert!(queue.enqueue(1).is_ok());
        assert!(queue.enqueue(2).is_ok());
        
        let result = queue.enqueue(3);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Queue overflow"));
        assert_eq!(queue.len(), 2); // Размер не должен измениться
    }

    #[test]
    fn test_dequeue() {
        let mut queue: LimitedQueue<i8> = LimitedQueue::new(3);
        
        queue.enqueue(1).unwrap();
        queue.enqueue(2).unwrap();
        queue.enqueue(3).unwrap();
        
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.len(), 2);
        
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.len(), 1);
        
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());
        
        assert_eq!(queue.dequeue(), None); // Пустая очередь
    }

    #[test]
    fn test_peek() {
        let mut queue: LimitedQueue<i8> = LimitedQueue::new(3);
        
        // Peek на пустой очереди
        assert_eq!(queue.peek(), None);
        
        queue.enqueue(1).unwrap();
        assert_eq!(queue.peek(), Some(&1));
        
        queue.enqueue(2).unwrap();
        assert_eq!(queue.peek(), Some(&2)); // Peek возвращает последний элемент
        
        queue.enqueue(3).unwrap();
        assert_eq!(queue.peek(), Some(&3));
        
        // После dequeue peek должен показывать новый последний элемент
        queue.dequeue();
        assert_eq!(queue.peek(), Some(&3));
    }

    #[test]
    fn test_is_empty() {
        let mut queue: LimitedQueue<i8> = LimitedQueue::new(2);
        
        assert!(queue.is_empty());
        
        queue.enqueue(1).unwrap();
        assert!(!queue.is_empty());
        
        queue.dequeue();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_mixed_operations() {
        let mut queue: LimitedQueue<i8> = LimitedQueue::new(3);
        
        // Добавляем и удаляем элементы
        queue.enqueue(1).unwrap();
        queue.enqueue(2).unwrap();
        assert_eq!(queue.dequeue(), Some(1));
        
        queue.enqueue(3).unwrap();
        queue.enqueue(4).unwrap();
        
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.peek(), Some(&4));
        
        // Проверяем порядок извлечения
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_zero_capacity() {
        let mut queue: LimitedQueue<i8> = LimitedQueue::new(0);
        
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());
        
        let result = queue.enqueue(1);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Queue overflow"));
        
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.peek(), None);
    }

    #[test]
    fn test_string_items() {
        let mut queue = LimitedQueue::new(2);
        
        assert!(queue.enqueue("hello".to_string()).is_ok());
        assert!(queue.enqueue("world".to_string()).is_ok());
        
        assert_eq!(queue.peek(), Some(&"world".to_string()));
        assert_eq!(queue.dequeue(), Some("hello".to_string()));
        assert_eq!(queue.dequeue(), Some("world".to_string()));
    }

    #[test]
    fn test_new_stack() {
        let stack: LimitedStack<i8> = LimitedStack::new(5);
        assert_eq!(stack.max_size, 5);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_push_and_pop() {
        let mut stack: LimitedStack<i8> = LimitedStack::new(3);
        
        assert!(stack.push(1).is_ok());
        assert!(stack.push(2).is_ok());
        assert!(stack.push(3).is_ok());
        
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_push_overflow() {
        let mut stack: LimitedStack<i8> = LimitedStack::new(2);
        
        assert!(stack.push(1).is_ok());
        assert!(stack.push(2).is_ok());
        
        let result = stack.push(3);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Stack overflow"));
    }

    #[test]
    fn test_stack_peek() {
        let mut stack: LimitedStack<i8> = LimitedStack::new(3);
        
        // Peek на пустом стеке
        assert_eq!(stack.peek(), None);
        
        stack.push(1).unwrap();
        assert_eq!(stack.peek(), Some(&1));
        
        stack.push(2).unwrap();
        assert_eq!(stack.peek(), Some(&2));
        
        stack.push(3).unwrap();
        assert_eq!(stack.peek(), Some(&3));
        
        // Peek не должен удалять элемент
        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.pop(), Some(3));
    }

    #[test]
    fn test_stack_is_empty() {
        let mut stack: LimitedStack<i8> = LimitedStack::new(2);
        
        assert!(stack.is_empty());
        
        stack.push(1).unwrap();
        assert!(!stack.is_empty());
        
        stack.pop();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_lifo_behavior() {
        let mut stack: LimitedStack<i8> = LimitedStack::new(3);
        
        stack.push(1).unwrap();
        stack.push(2).unwrap();
        stack.push(3).unwrap();
        
        // Проверяем порядок LIFO (Last-In-First-Out)
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_stackmixed_operations() {
        let mut stack: LimitedStack<i8> = LimitedStack::new(3);
        
        // Добавляем и удаляем элементы
        stack.push(1).unwrap();
        stack.push(2).unwrap();
        assert_eq!(stack.pop(), Some(2));
        
        stack.push(3).unwrap();
        stack.push(4).unwrap();
        
        assert_eq!(stack.peek(), Some(&4));
        
        // Проверяем порядок извлечения
        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_stack_zero_capacity() {
        let mut stack: LimitedStack<i8> = LimitedStack::new(0);
        
        assert!(stack.is_empty());
        
        let result = stack.push(1);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Stack overflow"));
        
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.peek(), None);
    }

    #[test]
    fn test_stack_string_items() {
        let mut stack = LimitedStack::new(2);
        
        assert!(stack.push("hello".to_string()).is_ok());
        assert!(stack.push("world".to_string()).is_ok());
        
        assert_eq!(stack.peek(), Some(&"world".to_string()));
        assert_eq!(stack.pop(), Some("world".to_string()));
        assert_eq!(stack.pop(), Some("hello".to_string()));
    }

    #[test]
    fn test_capacity_after_operations() {
        let mut stack: LimitedStack<i8> = LimitedStack::new(3);
        
        // Заполняем стек
        stack.push(1).unwrap();
        stack.push(2).unwrap();
        stack.push(3).unwrap();
        
        // Освобождаем место
        stack.pop();
        
        // Снова можем добавлять
        assert!(stack.push(4).is_ok());
        
        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
    }

    #[test]
    fn test_multiple_overflow_attempts() {
        let mut stack: LimitedStack<i8> = LimitedStack::new(1);
        
        assert!(stack.push(1).is_ok());
        
        // Несколько попыток переполнения
        assert!(stack.push(2).is_err());
        assert!(stack.push(3).is_err());
        assert!(stack.push(4).is_err());
        
        // После ошибок стек должен оставаться в валидном состоянии
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        
        // Теперь снова можно добавлять
        assert!(stack.push(5).is_ok());
    }
}
