#![allow(dead_code)]
use mano::{cli, CommandOptions};

const VERSION: &str = "Mano v0.1.0";
const CONFIG_FILE: &str = "mano.config.json";

fn main() {
    // (1) work on the command line
    let cmd = cli();

    let cmd_options = CommandOptions {
        watch: cmd.get_one("watch").unwrap(),
        minify: cmd.get_one("minify").unwrap(),
        init: cmd.get_one("init").unwrap(),
        unused: cmd.get_one("unused").unwrap(),
        dev: cmd.get_one("dev").unwrap(),
        search: match cmd.get_one::<String>("search") {
            Some(query) => query,
            None => "NO QUERY",
        },
    };

    println!("{:#?}", cmd_options);
}
