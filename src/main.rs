#![allow(dead_code)]
use crate::matches::{cli, CommandOptions};

pub mod matches;

const CONFIG_FILE: &str = "mano.config.json";

fn main() {
    // (1) work on the command line
    let cmd = cli();

    let cmd_options = CommandOptions {
        watch: cmd.get_one::<bool>("watch").unwrap(),
        minify: cmd.get_one::<bool>("minify").unwrap(),
        init: cmd.get_one::<bool>("init").unwrap(),
        unused: cmd.get_one::<bool>("unused").unwrap(),
        dev: cmd.get_one::<bool>("dev").unwrap(),
        search: match cmd.get_one::<String>("search") {
            Some(query) => query,
            None => "NO QUERY",
        },
    };

    println!("{:#?}", cmd_options);
}
