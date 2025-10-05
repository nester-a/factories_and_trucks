mod collections;
mod models;

use models::{Factory, Warehouse, Truck, Product};

fn main() {
    let factory = Factory::new(String::from("first factory"));
    let product = factory.create();
    println!("factory produced {}", product.name);

    let mut warehouse = Warehouse::new(5);
    let _ = warehouse.load(product);
    println!("{} moved to warehouse", warehouse.last_cargo().name);
    println!("warehouse has {} cargo", warehouse.loaded());

    let mut truck = Truck::new(String::from("small truck"), 3);

    let product = match warehouse.unload(){
        Some(prd) => prd,
        None => panic!() // Нужно паниковать!
    };

    println!("{} moved out from warehouse", product.name);
    println!("warehouse has {} cargos", warehouse.loaded());

    let _ = truck.load(product);

    println!("{} moved to truck", truck.last_cargo().name);
    println!("truck has {} cargo", truck.loaded());

    let mut market: Vec<Product> = Vec::new();

    let product = match truck.unload(){
        Some(prd) => prd,
        None => panic!() // Нужно паниковать!
    };

    println!("{} moved out from truck", product.name);
    println!("truck has {} cargos", truck.loaded());

    market.push(product);
    println!("{} moved in market", market.last().unwrap().name);
    println!("market has {} products", market.len());
}
