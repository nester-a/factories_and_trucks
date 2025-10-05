mod collections;

use collections::{LimitedQueue, LimitedStack};

pub struct Product {
    name: String,
}

pub struct Factory {
    name: String,
}
impl Factory {
    pub fn create(&self) -> Product {
        Product {
            name: format!("product from {}", self.name),
        }
    }
}

pub struct Truck {
    name: String,
    capacity: usize,
    cargo: LimitedStack<Product>
}
impl Truck{
    pub fn new(name: String, capacity: usize) -> Self {
        Self{
            name,
            capacity,
            cargo: LimitedStack<Product>(capacity)
        }
    }
    pub fn load(&self, product: Product) {
        self.cargo.push(product);
    }
    pub fn uload(&self) -> Product{
        self.cargo.pop()
    }
}

pub struct Warehouse {
    capacity: usize,
    cargo: LimitedQueue<Product>
}

impl Warehouse{
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            cargo: LimitedQueue<Product>(capacity)
        }
    }
}

