#![feature(phase)]
#[phase(plugin)] extern crate bindgen;

#[allow(dead_code, uppercase_variables, non_camel_case_types)]

use libc::*;

/* These are hacks to get around Bindgen being broken */
type va_list = ();
type Struct_st_table = ();

bindgen!(
    "/Users/jroesch/.rbenv/versions/2.1.3/include/ruby-2.1.0/ruby.h",
    match="ruby.h",
    link_framework="ruby",
    emit_builtins=true,
    allow_unknown_types=true,
    clang_args="-I/Users/jroesch/.rbenv/versions/2.1.3/include/ruby-2.1.0"
)
