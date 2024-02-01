pub fn hello_world() {
    println!("Hello, world!");
}

#[repr(u64)]
enum A {
    a1 = 0,
    a2 = 2,
    a3,
    a4 = 5,
}

#[repr(C)]
pub struct MyStruct {
    number: Box<i32>,
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct");
    }
}

/// This function takes a box and does nothing with it.
///
/// @param with_box The box to be dropped.
#[no_mangle]
pub extern "C" fn root(with_box: &MyStruct) {}

/// This function takes a box and drops it.
#[no_mangle]
pub extern "C" fn drop_box(x: Box<i32>) {}

#[no_mangle]
pub extern "C" fn drop_my_struct(x: Box<MyStruct>) {}

/// This function takes an optional box and drops it if it is not None.
#[no_mangle]
pub extern "C" fn drop_box_opt(x: Option<Box<i32>>) {}
