#![allow(dead_code)]

use mano::NothingForNow;

fn main() {
    let a = NothingForNow {
        nothing: String::from("hello mano"),
        _for: String::from("am gonna"),
        now: String::from("build you"),
    };

    println!("{:#?}", a);
}
