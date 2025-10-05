mod limited_vecs;

use limited_vecs::{LimitedQueue, LimitedStack};

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
    max_size: usize,
    cargo: LimitedStack<Product>
}
impl Truck{
    pub fn new(name: String, max_size: usize) -> Self {
        Self{
            name,
            max_size,
            cargo: LimitedStack<Product>(max_size)
        }
    }
    pub fn load(&self, product: Product) {
        self.cargo.push(product);
    }
    pub fn uload(&self) -> Product{
        self.cargo.pop()
    }
}

