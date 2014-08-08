extern crate verdigris;

use verdigris::gl;

fn main() {
    if gl::get_proc_address("hello").is_null() {
        println!("symbol don't exist");
    } else {
        println!("should not append");
    }

    if gl::get_proc_address("glClear").is_null() {
        println!("should not append");
    } else {
        println!("Yeah !");
    }
}