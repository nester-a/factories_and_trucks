mod collections;
mod models;

use collections::LimitedQueue;
use models::Truck;

fn main() {
    let queue: LimitedQueue<u8> = LimitedQueue::new(2);
    let mut st = format!("{}", queue.max_size);
    println!("{}", st);
    st = format!("{}", queue.len());
    println!("{}", st);
}
