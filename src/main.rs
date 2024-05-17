#![allow(dead_code)]
use mano::{cli, CommandOptions};

const VERSION: &str = "Mano v0.1.0";
const CONFIG_FILE: &str = "mano.config.json";

fn main() {
    // (1) work on the command line
    let cmd = cli();

    let cmd_options = CommandOptions {
        watch: cmd.get_one::<bool>("watch").unwrap(),
        minify: cmd.get_one::<bool>("minify").unwrap(),
        init: cmd.get_one::<bool>("init").unwrap(),
        unused: cmd.get_one::<bool>("unused").unwrap(),
        browser: cmd.get_one::<bool>("browser").unwrap(),
        search: match cmd.get_one::<String>("search") {
            /* we need a Vec */
            Some(query) => query,
            None => "NO QUERY",
        },
    };

    println!("{:#?}", cmd_options);
}
