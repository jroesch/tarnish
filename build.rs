extern crate bindgen;

use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut file = OpenOptions::new()
                   .write(true)
                   .truncate(true)
                   .create(true)
                   .open("src/raw/ruby.rs")
                   .unwrap();

    let mut builder = bindgen::builder();
    // builder.forbid_unknown_types();
    // builder.emit_builtins();
    builder.header("/Users/jroesch/.rbenv/versions/2.2.2/include/ruby-2.2.0/ruby.h");
    builder.match_pat("ruby");
    builder.match_pat("select");
    builder.link_framework("ruby");
    builder.clang_arg("-I/Users/jroesch/.rbenv/versions/2.2.2/include/ruby-2.2.0");
    builder.clang_arg("-I/Users/jroesch/.rbenv/versions/2.2.2/include/ruby-2.2.0/x86_64-darwin14");

    file.write("use libc::{size_t, uint32_t};\n".as_bytes()).unwrap();
    file.write("use libc::types::os::arch::posix88::pid_t;\n".as_bytes()).unwrap();
    file.write("use std::os::unix::raw::{time_t, mode_t};\n".as_bytes()).unwrap();
    file.write("use libc::types::os::common::posix01::timeval;\n".as_bytes()).unwrap();
    file.write("pub type Struct_timeval = timeval;\n".as_bytes()).unwrap();
    file.write("pub type va_list = ();".as_bytes()).unwrap();

    let bindings = builder.generate().unwrap();
    bindings.write(Box::new(file)).unwrap();
}
