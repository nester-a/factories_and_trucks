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
    cargo: LimitedStack<Product>,
}

impl Truck {
    pub fn new(name: String, capacity: usize) -> Self {
        Self {
            name,
            capacity,
            cargo: LimitedStack::new(capacity),
        }
    }

    pub fn load(&mut self, product: Product) -> Result<(), &'static str> {
        self.cargo.push(product)
    }

    pub fn unload(&mut self) -> Option<Product> {
        self.cargo.pop()
    }

    pub fn loaded(&self) -> usize {
        self.cargo.len()
    }
}

pub struct Warehouse {
    capacity: usize,
    cargo: LimitedQueue<Product>,
}

impl Warehouse {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            cargo: LimitedQueue::new(capacity),
        }
    }


    pub fn load(&mut self, product: Product) -> Result<(), &'static str> {
        self.cargo.enqueue(product)
    }

    pub fn unload(&mut self) -> Option<Product> {
        self.cargo.dequeue()
    }

    pub fn loaded(&self) -> usize {
        self.cargo.len()
    }
}

#[cfg(test)]
mod tests {}
use super::*;

#[test]
fn test_factory_create() {
    let factory = Factory {
        name: "TestFactory".to_string(),
    };

    let product = factory.create();
    assert_eq!(product.name, "product from TestFactory");
}

#[test]
fn test_truck_new() {
    let truck = Truck::new("Truck1".to_string(), 10);

    assert_eq!(truck.name, "Truck1");
    assert_eq!(truck.capacity, 10);
    assert!(truck.cargo.is_empty());
}

#[test]
fn test_truck_load_and_unload() {
    let mut truck = Truck::new("Truck1".to_string(), 3);
    let factory = Factory {
        name: "Factory1".to_string(),
    };

    let product1 = factory.create();
    let product2 = factory.create();

    // Загрузка продуктов
    assert!(truck.load(product1).is_ok());
    assert!(truck.load(product2).is_ok());
    assert_eq!(truck.loaded(), 2);

    // Выгрузка (LIFO для стека)
    let unloaded_product2 = truck.unload().unwrap();
    assert_eq!(unloaded_product2.name, "product from Factory1");
    assert_eq!(truck.loaded(), 1);

    let unloaded_product1 = truck.unload().unwrap();
    assert_eq!(unloaded_product1.name, "product from Factory1");
    assert!(truck.cargo.is_empty());
}

#[test]
fn test_truck_load_overflow() {
    let mut truck = Truck::new("SmallTruck".to_string(), 1);
    let factory = Factory {
        name: "Factory1".to_string(),
    };

    let product1 = factory.create();
    let product2 = factory.create();

    assert!(truck.load(product1).is_ok());

    let result = truck.load(product2);
    assert!(result.is_err());
    assert_eq!(result.err(), Some("Stack overflow"));
}

#[test]
fn test_truck_unload_empty() {
    let mut truck = Truck::new("EmptyTruck".to_string(), 5);

    assert!(truck.unload().is_none());
}

#[test]
fn test_warehouse_new() {
    let warehouse = Warehouse::new(20);

    assert_eq!(warehouse.capacity, 20);
    assert!(warehouse.cargo.is_empty());
}

#[test]
fn test_warehouse_operations() {
    let mut warehouse = Warehouse::new(3);
    let factory = Factory {
        name: "MainFactory".to_string(),
    };

    let product1 = factory.create();
    let product2 = factory.create();
    let product3 = factory.create();

    // Добавление продуктов на склад
    assert!(warehouse.cargo.enqueue(product1).is_ok());
    assert!(warehouse.cargo.enqueue(product2).is_ok());
    assert!(warehouse.cargo.enqueue(product3).is_ok());
    assert_eq!(warehouse.loaded(), 3);

    // Извлечение (FIFO для очереди)
    let dequeued_product1 = warehouse.cargo.dequeue().unwrap();
    assert_eq!(dequeued_product1.name, "product from MainFactory");
    assert_eq!(warehouse.loaded(), 2);

    let dequeued_product2 = warehouse.cargo.dequeue().unwrap();
    assert_eq!(dequeued_product2.name, "product from MainFactory");
    assert_eq!(warehouse.loaded(), 1);
}

#[test]
fn test_warehouse_overflow() {
    let mut warehouse = Warehouse::new(1);
    let factory = Factory {
        name: "SmallFactory".to_string(),
    };

    let product1 = factory.create();
    let product2 = factory.create();

    assert!(warehouse.cargo.enqueue(product1).is_ok());

    let result = warehouse.cargo.enqueue(product2);
    assert!(result.is_err());
    assert_eq!(result.err(), Some("Queue overflow"));
}

#[test]
fn test_truck_to_warehouse_transfer() {
    let mut truck = Truck::new("DeliveryTruck".to_string(), 2);
    let mut warehouse = Warehouse::new(2);
    let factory = Factory {
        name: "DeliveryFactory".to_string(),
    };

    // Загружаем товар в грузовик
    let product1 = factory.create();
    let product2 = factory.create();

    assert!(truck.load(product1).is_ok());
    assert!(truck.load(product2).is_ok());

    // Разгружаем из грузовика на склад
    let product_from_truck = truck.unload().unwrap();
    assert!(warehouse.cargo.enqueue(product_from_truck).is_ok());

    let product_from_truck = truck.unload().unwrap();
    assert!(warehouse.cargo.enqueue(product_from_truck).is_ok());

    assert!(truck.cargo.is_empty());
    assert_eq!(warehouse.loaded(), 2);
}

#[test]
fn test_product_creation_different_factories() {
    let factory1 = Factory {
        name: "FactoryA".to_string(),
    };
    let factory2 = Factory {
        name: "FactoryB".to_string(),
    };

    let product1 = factory1.create();
    let product2 = factory2.create();

    assert_eq!(product1.name, "product from FactoryA");
    assert_eq!(product2.name, "product from FactoryB");
}

#[test]
fn test_truck_peek_cargo() {
    let mut truck = Truck::new("PeekTruck".to_string(), 3);
    let factory = Factory {
        name: "TestFactory".to_string(),
    };

    let product1 = factory.create();
    let product2 = factory.create();

    truck.load(product1).unwrap();
    truck.load(product2).unwrap();

    // Peek должен показывать последний загруженный продукт (LIFO)
    let last_product = truck.cargo.peek().unwrap();
    assert_eq!(last_product.name, "product from TestFactory");

    // Peek не должен удалять продукт
    assert_eq!(truck.loaded(), 2);
}

#[test]
fn test_warehouse_peek() {
    let mut warehouse = Warehouse::new(2);
    let factory = Factory {
        name: "TestFactory".to_string(),
    };

    let product1 = factory.create();
    let product2 = factory.create();

    warehouse.cargo.enqueue(product1).unwrap();
    warehouse.cargo.enqueue(product2).unwrap();

    // Peek должен показывать первый продукт (FIFO)
    let first_product = warehouse.cargo.peek().unwrap();
    assert_eq!(first_product.name, "product from TestFactory");

    // Peek не должен удалять продукт
    assert_eq!(warehouse.loaded(), 2);
}

#[test]
fn test_multiple_trucks() {
    let mut truck1 = Truck::new("Truck1".to_string(), 2);
    let mut truck2 = Truck::new("Truck2".to_string(), 2);
    let factory = Factory {
        name: "MultiFactory".to_string(),
    };

    let product1 = factory.create();
    let product2 = factory.create();

    // Загрузка в разные грузовики
    assert!(truck1.load(product1).is_ok());
    assert!(truck2.load(product2).is_ok());

    assert_eq!(truck1.loaded(), 1);
    assert_eq!(truck2.loaded(), 1);

    // Выгрузка из разных грузовиков
    let from_truck1 = truck1.unload().unwrap();
    let from_truck2 = truck2.unload().unwrap();

    assert_eq!(from_truck1.name, "product from MultiFactory");
    assert_eq!(from_truck2.name, "product from MultiFactory");
    assert!(truck1.cargo.is_empty());
    assert!(truck2.cargo.is_empty());
}
