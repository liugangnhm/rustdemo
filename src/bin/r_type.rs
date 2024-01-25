trait Foo {
    type R;
    fn foo(&self) -> Self::R;
}

struct Foo1 {}
struct Fool2 {}

impl Foo for Foo1 {
    type R = i32;
    fn foo(&self) -> Self::R {
        1
    }
}

impl Foo for Fool2 {
    type R = f64;

    fn foo(&self) -> Self::R {
        8f64 * 2f64 * 1.0
    }
}

fn main() {
    let f1 = Foo1 {};
    println!("{}", f1.foo());

    let f2 = Fool2 {};
    println!("{}", f2.foo());
}
