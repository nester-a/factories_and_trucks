mod limited_vecs;

use limited_vecs::{LimitedQueue, LimitedStack};

fn main() {
    let queue: LimitedQueue<u8> = LimitedQueue::new(2);
    let mut st = format!("{}", queue.max_size);
    println!("{}", st);
    st = format!("{}", queue.len());
    println!("{}", st);
}
