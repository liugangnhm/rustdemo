use std::cell::Cell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq)]
struct BadKey {
    value: Cell<i32>,
}

impl BadKey {
    fn new(v: i32) -> Self {
        BadKey {
            value: Cell::new(v),
        }
    }
}

impl Hash for BadKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.get().hash(state);
    }
}

fn main() {
    let mut map = HashMap::new();
    map.insert(BadKey::new(1), 100);
    map.insert(BadKey::new(2), 200);

    println!("Before Find key 1:{:? }", map.get(&BadKey::new(1)));
    println!("Before Find key 2:{:? }", map.get(&BadKey::new(2)));
    println!("Before Find key 4:{:? }", map.get(&BadKey::new(4)));

    for key in map.keys() {
        key.value.set(key.value.get() * 2);
    }

    println!("Find key 1:{:? }", map.get(&BadKey::new(1)));
    println!("Find key 2:{:? }", map.get(&BadKey::new(2)));
    println!("Find key 4:{:? }", map.get(&BadKey::new(4)));
}
