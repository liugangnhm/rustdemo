fn fib(index: u32) -> u64 {
    if index == 1 || index == 2 {
        1
    } else {
        fib(index - 1) + fib(index - 2)
    }
}
fn main() {
    let f8 = fib(100);
    println!("{}", f8);
}
