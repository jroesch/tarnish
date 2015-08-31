use raw;
use super::ruby::ToValue;
use std::mem::transmute;

#[macro_export]
macro_rules! ruby_extension {
    ($name:ident, $body:block) => (
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn $name() {
            $body
        }
    );
}

#[macro_export]
macro_rules! define_module_fn {
    ($this:expr, $name:expr, $arity:expr, $args:ident $body:block) => (
        {
            extern "C" fn generated_fn(
                argc: libc::c_int,
                arg1: *mut libc::c_void,
                arg2: *mut libc::c_void,
                arg3: *mut libc::c_void,
                ) -> $crate::ruby::Value {
                // setup args
                let $args = if (argc >= 0) && (argc <= 17) {
                    println!("case one");
                    Args::Precise(Vec::new())
                } else if argc < 0 {
                    if argc == -1 {
                        println!("case two");
                        Args::Precise(Vec::new())
                    } else if argc == -2 {
                        println!("case three");
                        Args::Precise(Vec::new())
                    } else {
                        panic!("invalid argument count: {}", argc);
                    }
                } else {
                    panic!("invalid argument count: {}", argc);
                };

                return $body;
            }

            $this.define_module_fn(
                $name,
                $arity,
                unsafe { std::mem::transmute(generated_fn) })
        }
    )
}
