#![allow(dead_code)]

// (1) The first task of this project,
// will be the creation of a robust command line interface.

#[allow(unused_imports)]
use ::clap::{
    arg,
    builder::{styling::AnsiColor, Styles},
    value_parser, Arg, ArgAction, ArgMatches, Command,
};

#[derive(Debug)]
pub struct CommandOptions<B /* bool */, A /* Array */> {
    pub watch: B,
    pub init: B,
    pub minify: B,
    pub unused: B,
    pub browser: B,
    pub search: A,
}

impl<B, A> CommandOptions<B, A> {}

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
            arg!(-w --watch "Watches the target file(s) for changes"),
            arg!(-m --minify "Minifies the output file(s)"),
            arg!(-i --init "Creates a json file with specific settings"),
            arg!(-u --unused "Clean up unsued variables and style declarations"),
            Arg::new("search")
                .short('s')
                .long("search")
                .value_parser(value_parser!(String))
                .action(ArgAction::Set)
                .value_delimiter(',')
                .num_args(1..3)
                .help("Searches the program for utility classes, fonts, colors, and more"),
            arg!(-b --browser "Opens a new browser window, displaying all current activities"),
        ])
        .styles(display_styles)
        .arg_required_else_help(true) // exp
        .get_matches();

    matches
}

// The second task of this project will be the implementation of useful filesystem functions
