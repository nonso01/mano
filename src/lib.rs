#![allow(dead_code)]
const VERSION: &str = "v0.1.0";

// (1) The first task of this project,
// will be the creation of a robust command line interface.

#[allow(unused_imports)]
use ::clap::{
    arg,
    builder::{styling::AnsiColor, Styles},
    value_parser, Arg, ArgAction, ArgMatches, Command,
};

pub fn cli() -> ArgMatches {
    let about_crate = String::from("Mano is a powerful and blazing-fast utility-first css generation library\nwritten in Rust, it offers significant enhancement in performance, flexibility and feature set.");

    let display_styles = Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Blue.on_default())
        .literal(AnsiColor::Cyan.on_default());

    let matches = Command::new("mano")
        .author("Nonso Martin")
        .version(env!("CARGO_PKG_VERSION"))
        .about(&about_crate)
        .args([
            arg!(-w --watch "Watches the target file(s) for changes")
                .value_parser(value_parser!(bool)),
            arg!(-m --minify "Minifies the output file(s)"),
            arg!(-i --init "Creates a json file with specific settings"),
            arg!(-u --unused "Clean up unsued variables and style declarations"),
            Arg::new("search")
                .short('s')
                .long("search")
                .action(ArgAction::Set)
                .value_delimiter(',')
                .num_args(1..5)
                .help("Searches the program for utility classes, fonts, colors, and more"),
            arg!(-t --tab "Opens a new browser window, displaying all current activities"),
        ])
        .styles(display_styles)
        .arg_required_else_help(true) // exp
        .get_matches();

    matches
}

// The second task of this project will be the implementation of useful filesystem functions
