use raw;
use super::ruby::ToValue;

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
    ($this:expr, $name:expr, $arity:expr, $body:ident) => (
        {
            $this.define_module_fn(
                $name,
                $arity,
                unsafe { std::mem::transmute($body) })
        }
    )
}
