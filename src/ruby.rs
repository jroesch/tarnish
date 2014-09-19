use raw::ruby::VALUE;
use raw::ruby::ID;
use libc::c_int;
use raw::ruby;

pub fn symbol<C: ToCStr>(symbol_text: C) -> Symbol {
    let c_string = symbol_text.to_c_str().as_ptr();
    Symbol(unsafe { ruby::rb_intern(c_string) })
}

pub fn method_call(
  receiver: Object, method_name: Symbol,
  arity: c_int, args: Vec<Object>
) -> Object {
  match receiver {
    Value(recv) =>
      Value(unsafe {
        ruby::rb_funcall(recv, method_name.to_id(), arity)
      })
  }
}

struct Symbol(ID);

impl Symbol {
  fn to_id(&self) -> ID { match self { &Symbol(id) => id } }
}

enum Object {
  Value(VALUE)
}

fn method(name: String) {
  let method_sym = symbol("puts");

  //rb_function
}

#[test]
fn testing() {
  
}
