#![feature(libc)]
#![feature(concat_idents)]
extern crate libc;

use std::ffi::CString;

mod raw {
    pub mod ruby;
}

pub mod ruby;
#[macro_use] pub mod macros;

use ruby::*;

ruby_extension!(Init_tarnish, {
    let mut module = Module::new(CString::new("MyVeryFooBar").unwrap());
    define_module_fn!(module, CString::new("test").unwrap(), 0, args {
        match args { _ => panic!() }
        return raw::ruby::RUBY_Qnil as u64;
    });
    println!("After module decl!")
});
