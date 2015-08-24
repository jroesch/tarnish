#![crate_name="tarnish"]
#![crate_type="dylib"]

extern crate libc;

pub mod raw {
  pub mod ruby;
}

pub mod ruby;

#[test]
fn it_works() {
}

// static mut RubyModuleRust: raw::ruby::VALUE =

#[no_mangle]
pub extern fn Init_tarnish() {
    println!("Hello World!");
    //let foo = "FooBar".to_c_str().as_ptr();
    let f1 = unsafe { raw::ruby::rb_exit(0) };
    //unsafe { raw::ruby::rb_intern(foo) };
    // let res = unsafe { raw::ruby::rb_define_module("MyVeryFooBar".to_c_str().as_ptr()) };
    println!("After module decl!")
}
