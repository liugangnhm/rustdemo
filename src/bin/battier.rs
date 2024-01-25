use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    let barrier = Arc::new(Barrier::new(10));
    let mut handlers = vec![];
    for i in 0..10 {
        let c = barrier.clone();
        // The same messages will be printed together.
        // You will NOT see any interleaving.
        let t = thread::spawn(move || {
            println!("before wait {}", i);
            c.wait();
            println!("after wait {}", i);
        });
        handlers.push(t);
    }

    // for h in handlers {
    //     h.join().ok();
    // }

    // sleep 3s
    std::thread::sleep(std::time::Duration::from_millis(3000));
}
