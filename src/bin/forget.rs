use std::{fs::File, mem};

fn main() {
    let file = File::open("foo.txt").unwrap();
    mem::forget(file);
}
