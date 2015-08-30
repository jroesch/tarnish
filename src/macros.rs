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
