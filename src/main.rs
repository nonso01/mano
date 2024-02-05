#![allow(dead_code)]
use mano::mano_cli::{DefaultOptions, CommandOptions};

fn main() {
    let a = CommandOptions {
        help: String::from("help"),
        version: String::from("mano v0.1.0"),
        init: true,
        minify: true,
        clean: false,
        watch: false,
        update: true,
    };

    println!("{:#?}", a.display_help());
}
