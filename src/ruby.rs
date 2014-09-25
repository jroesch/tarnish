use raw::ruby::VALUE;
use raw::ruby::ID;
use libc::c_int;
use raw::ruby;

pub fn symbol<C: ToCStr>(symbol_text: C) -> Symbol {
    let c_string = symbol_text.to_c_str().as_ptr();
    Symbol(unsafe { ruby::rb_intern(c_string) })
}

macro_rules! raw_ruby_funcall(
    ($recv:expr, $mname:expr, $arity:expr, $($args:expr)*) =>
         (unsafe { ruby::rb_funcall($recv, $mname, $arity, $($args)*) })
)

pub fn method_call(
  receiver: Object, method_name: Symbol, args: Vec<Object>
) -> Object {
  // let raw_args: Vec<Value> =
  match receiver {
    Value(recv) =>
      Value(unsafe {
        raw_ruby_funcall!(recv, method_name.to_id(), (args.len() as i32), args)
      })
  }
}

pub struct Symbol(ID);

impl Symbol {
  fn to_id(&self) -> ID { match self { &Symbol(id) => id } }
}

pub enum Object {
  Value(VALUE)
}

fn method(name: String) {
  let method_sym = symbol("puts");

  //rb_function
}

#[test]
fn testing() {
    //method_call(Value(ruby::rb_cObject), symbol("puts"), vec![symbol("hello")]);
    raw_ruby_funcall!(ruby::rb_cObject, symbol("puts").to_id(), 1, symbol("hello").to_id());
}
