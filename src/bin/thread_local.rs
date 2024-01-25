use std::cell::RefCell;
use std::sync::Mutex;
use std::thread;

static FOO2: Mutex<RefCell<u32>> = Mutex::new(RefCell::new(1));

fn main() {
    thread_local! {
        static FOO: RefCell<u32> = RefCell::new(1)
    };

    *FOO2.lock().unwrap().borrow_mut() = 2;

    FOO.with(|f| {
        println!("main thread value1 {:?}", *f.borrow());
        *f.borrow_mut() = 2;
        println!("main thread value2 {:?}", *f.borrow());

        *FOO2.lock().unwrap().borrow_mut() = 4;
    });

    let t = thread::spawn(move || {
        FOO.with(|f| {
            println!("child thread value1 {:?}", *f.borrow());
            *f.borrow_mut() = 3;
            println!("child thread value2 {:?}", *f.borrow());
        });
    });
    t.join().ok();

    FOO.with(|f| {
        println!("main thread value3 {:?}", *f.borrow());
    });
}
