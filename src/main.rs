#![allow(dead_code)]
use mano::mano_cli::{CommandLineOperations, CommandLineOptions};

fn main() {
    let a = CommandLineOptions {
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
