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

    // need to clean this code up to not just work on my one mac
    let mut builder = bindgen::builder();
    // builder.forbid_unknown_types();
    // builder.emit_builtins();
    let ruby_path = "/Users/jroesch/.rbenv/versions/2.2.2";
    builder.header("/Users/jroesch/.rbenv/versions/2.2.2/include/ruby-2.2.0/ruby.h");
    builder.match_pat("ruby");
    builder.match_pat("select.h");
    builder.link("ruby");
    builder.clang_arg("-L/Users/jroesch/.rbenv/versions/2.2.2/lib");
    builder.clang_arg("-I/Users/jroesch/.rbenv/versions/2.2.2/include/ruby-2.2.0");
    builder.clang_arg("-I/Users/jroesch/.rbenv/versions/2.2.2/include/ruby-2.2.0/x86_64-darwin14");

    file.write("#![allow(dead_code)]\n".as_bytes()).unwrap();
    file.write("#![allow(non_camel_case_types)]\n".as_bytes()).unwrap();
    file.write("#![allow(non_upper_case_globals)]\n".as_bytes()).unwrap();
    file.write("use libc::{size_t, uint32_t, int32_t};\n".as_bytes()).unwrap();
    file.write("use libc::types::os::arch::posix88::pid_t;\n".as_bytes()).unwrap();
    file.write("use std::os::unix::raw::{time_t, mode_t};\n".as_bytes()).unwrap();
    file.write("use libc::types::os::common::posix01::{timeval, timespec};\n".as_bytes()).unwrap();
    file.write("pub type Struct_timeval = timeval;\n".as_bytes()).unwrap();
    file.write("pub type Struct_timespec = timespec;\n".as_bytes()).unwrap();
    file.write("pub type fd_set = int32_t;\n".as_bytes()).unwrap();
    file.write("pub type sigset_t = int32_t;\n".as_bytes()).unwrap();
    file.write("pub type va_list = int32_t;".as_bytes()).unwrap();

    let bindings = builder.generate().unwrap();
    bindings.write(Box::new(file)).unwrap();
    println!("cargo:rustc-link-search=native=/Users/jroesch/.rbenv/versions/2.2.2/lib")
}
