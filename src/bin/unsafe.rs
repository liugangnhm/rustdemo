fn raw_to_ref<'a>(p: *const i32) -> &'a i32 {
    unsafe { &*p }
}
fn main() {
    let a: u32 = 42;
    let p: &i32 = raw_to_ref(&a as *const u32 as *const i32);
    println!("{}", p);
}
