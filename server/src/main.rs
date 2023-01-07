#![allow(dead_code, unused_imports)]
#![warn(
    clippy::wildcard_imports,
    clippy::string_add,
    clippy::string_add_assign,
    clippy::manual_ok_or,
    unused_lifetimes
)]

pub mod actor;
pub mod websockets;

fn main() {
    println!("Hello, world!");
}
