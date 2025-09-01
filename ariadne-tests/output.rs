#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use ariadne::define_as_grid;
use ariadne;
pub struct MyEntity {
    pub id: i64,
}
impl MyEntity {
    fn new() -> Self {
        return MyEntity { id: 1 };
    }
}
pub struct MyEntityGrid {
    pub list: Vec<MyEntity>,
}
impl MyEntityGrid {}
fn main() {
    {
        ::std::io::_print(format_args!("Hello, world!\n"));
    };
}
