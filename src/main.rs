#![allow(dead_code)]
use mano::{cli, CommandOptions};

const CONFIG_FILLE: &str = "mano.config.json";

fn main() {
    // (1) work on the command line
    let _cmd = cli();

    let _cmd_options = CommandOptions {
        watch: _cmd.get_one::<bool>("watch").unwrap(),
        minify: _cmd.get_one::<bool>("minify").unwrap(),
        init: _cmd.get_one::<bool>("init").unwrap(),
        unused: _cmd.get_one::<bool>("unused").unwrap(),
        browser: _cmd.get_one::<bool>("browser").unwrap(),
        search: match _cmd.get_one::<String>("search") {
            /* we need a Vec */
            Some(query) => query,
            None => "NO QUERY",
        },
    };

    println!("{:#?}", _cmd);
}
