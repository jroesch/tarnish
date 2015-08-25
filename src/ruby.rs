use raw::ruby::VALUE;
use raw::ruby::ID;
use libc::c_int;
use raw::ruby;
use std::ffi::{CString, CStr};

pub trait ToCString {
    #[inline]
    fn to_c_string(self) -> CString;
}

impl ToCString for CString {
    fn to_c_string(self) -> CString {
        self
    }
}

pub struct Class {
    class_object: VALUE,
}

pub struct Module {
    module_object: VALUE,
}

pub struct Symbol {
    id: ID,
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

// type MethodImpl = extern fn(ArgSequence) -> Result<Value, ()>;


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
    pub fn define_module_fn<C: ToCString>(&mut self, name: C, arity: c_int) {
        unsafe {
            ruby::rb_define_module_function(self.module_object, name.to_c_string().as_ptr(), Some(println), 0)
        }
    }
}

extern "C" fn println() -> VALUE {
    println!("hello from rust");
    unsafe { ruby::rb_ary_new() }
}

// pub fn symbol<C: ToCStr>(symbol_text: C) -> Symbol {
//     let c_string = symbol_text.to_c_str().as_ptr();
//     Symbol(unsafe { ruby::rb_intern(c_string) })
// }
//
// macro_rules! raw_ruby_funcall(
//     ($recv:expr, $mname:expr, $arity:expr, $($args:expr)*) =>
//          (unsafe { ruby::rb_funcall($recv, $mname, $arity, $($args)*) })
// );
//
// pub fn method_call(
//   receiver: Object, method_name: Symbol, args: Vec<Object>
// ) -> Object {
//   // let raw_args: Vec<Value> =
//   match receiver {
//     Value(recv) =>
//       Value(unsafe {
//         raw_ruby_funcall!(recv, method_name.to_id(), (args.len() as i32), args)
//       })
//   }
// }
//
// pub struct Symbol(ID);
//
// impl Symbol {
//   fn to_id(&self) -> ID { match self { &Symbol(id) => id } }
// }
//
// pub enum Object {
//   Value(VALUE)
// }
//
// fn method(name: String) {
//   let method_sym = symbol("puts");
//
//   //rb_function
// }
//
// #[test]
// fn testing() {
//     //method_call(Value(ruby::rb_cObject), symbol("puts"), vec![symbol("hello")]);
//     raw_ruby_funcall!(ruby::rb_cObject, symbol("puts").to_id(), 1, symbol("hello").to_id());
// }
