#![feature(phase)]
#[phase(plugin)] extern crate bindgen;

#[allow(dead_code, uppercase_variables, non_camel_case_types)]

use libc::*;

bindgen!(
    "/Users/jroesch/.rbenv/versions/2.1.2/include/ruby-2.1.0/ruby.h",
    match="ruby.h",
    link_static="ruby",
    emit_builtins=true,
    clang_args="-I/Users/jroesch/.rbenv/versions/2.1.2/include/ruby-2.1.0/"
)
