use libc::c_int;

use raw::ruby::VALUE;
use raw::ruby::ID;
use raw::ruby;

use std::mem::transmute;
use std::ffi::{CString, CStr};

pub const NIL: Value = 8;

pub trait ToCString {
    #[inline]
    fn to_c_string(self) -> CString;
}

impl ToCString for CString {
    fn to_c_string(self) -> CString {
        self
    }
}

// An opaque Ruby value
pub type Value = VALUE;

pub struct Class {
    class_object: Value,
}

pub struct Module {
    module_object: Value,
}

pub struct Symbol {
    id: ID,
}

pub trait ToValue {
    fn to_value(&self) -> Value;
}

impl ToValue for Class {
    fn to_value(&self) -> Value {
        self.class_object
    }
}

impl ToValue for Module {
    fn to_value(&self) -> Value {
        self.module_object
    }
}

impl ToValue for Value {
    fn to_value(&self) -> Value {
        *self
    }
}

impl Symbol {
    pub fn new<C: ToCString>(symbol: C) -> Symbol {
        unsafe {
            let c_string = symbol.to_c_string().as_ptr();
            Symbol {
                id: ruby::rb_intern(c_string)
            }
        }
    }
}

pub enum Args {
    Precise(Vec<Value>),
    ReceiverWithCArray(Value),
    ReceiverWithRArray(Value, Value),
}

impl Module {
    pub fn new<C: ToCString>(symbol: C) -> Module {
        unsafe {
            let c_string = symbol.to_c_string().as_ptr();
            Module {
                module_object: ruby::rb_define_module(c_string)
            }
        }
    }

    // I have a much more clever scheme in the future for determining arity automatically.
    pub fn define_module_fn(&mut self, name: CString,  arity: c_int, body: extern "C" fn(c_int) -> Value) {
        unsafe {
            ruby::rb_define_module_function(
                self.module_object,
                name.as_ptr(),
                Some(transmute(body)), arity)
        }
    }
}

// extern "C" fn println() -> VALUE {
//     println!("hello from rust");
//     unsafe { ruby::rb_ary_new() }
// }
