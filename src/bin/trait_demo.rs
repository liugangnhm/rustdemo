fn main() {
    let f: Box<i32> = Box::new(1);

    let f2 = f;
}

trait Foo {
    fn foo(&self) -> i32;
    fn foo2(self: Box<Self>) -> i32;
}
