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

#[no_mangle]
pub extern "C" fn create_my_struct_box() -> Box<MyStruct> {
    Box::new(MyStruct {
        number: Box::new(42),
    })
}

#[no_mangle]
pub extern "C" fn create_my_struct_ref() {}

/// This function takes a box and drops it.
#[no_mangle]
pub extern "C" fn drop_box(x: Box<i32>) {}

#[no_mangle]
pub extern "C" fn drop_my_struct(x: Box<MyStruct>) {
    println!("before Dropping box: {}", x.number);
}

/// This function takes an optional box and drops it if it is not None.
#[no_mangle]
pub extern "C" fn drop_box_opt(x: Option<Box<i32>>) {}

#[no_mangle]
pub extern "C" fn ffi_test() -> *mut u16 {
    let test = Box::new([11u16, 22, 33, 44]); // type must be explicit here...

    Box::into_raw(test) as *mut _ // ... because this cast can convert
                                  // *mut [i32; 4] to *mut u16
}

#[no_mangle]
pub extern "C" fn create_my_struct_array(prt: *mut *mut MyStruct, len: *mut i32) {
    let test = Box::new([
        MyStruct {
            number: Box::new(11),
        },
        MyStruct {
            number: Box::new(22),
        },
        MyStruct {
            number: Box::new(33),
        },
        MyStruct {
            number: Box::new(44),
        },
    ]);

    unsafe {
        *prt = Box::into_raw(test) as *mut _;
        *len = 4i32;
    }
}

#[no_mangle]
pub extern "C" fn drop_my_struct_array(prt: *mut MyStruct) {
    unsafe {
        let test = Box::from_raw(prt);
    }
}

#[no_mangle]
pub extern "C" fn ffi_test_2(p: *mut u16) {
    unsafe {
        let test = Box::from_raw(p);
        println!("{:?}", test);
    }
}
