#![allow(dead_code)]
use mano::mano_cli::{process_args};

fn main() {
    let a = process_args();
    println!("you passed\n {:?}", a);
}
