#![allow(dead_code)]

// (1) The first task of this project,
// will be the creation of a robust command line interface.

#[allow(unused_imports)]
use ::clap::{
    arg,
    builder::{
        styling::{self, AnsiColor},
        Styles,
    },
    value_parser, Arg, ArgAction, ArgMatches, Command,
};

pub const ABOUT_CRATE: &str = "Mano is a powerful and blazing-fast utility-first css generation library\nwritten in Rust, it offers significant enhancement in performance, flexibility and feature set.";

pub const SEARCH_PARAM: [&str; 5] = ["fonts", "text", "grid", "flex", "colors"];

#[derive(Debug)]
pub struct CommandOptions<B /* bool */, A /* String */> {
    pub watch: B,
    pub init: B,
    pub minify: B,
    pub unused: B,
    pub dev: B,
    pub search: A,
}

impl<B, A> CommandOptions<B, A> {}

pub fn cli() -> ArgMatches {
    let display_styles = Styles::styled()
        .header(
            AnsiColor::Yellow.on_default() | styling::Effects::BOLD | styling::Effects::UNDERLINE,
        )
        .usage(AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .literal(AnsiColor::Cyan.on_default() | styling::Effects::BOLD);

    let matches = Command::new("mano")
        .author("Nonso Martin")
        .version(env!("CARGO_PKG_VERSION"))
        .about(ABOUT_CRATE)
        .args([
            arg!(-w --watch "Watches the target file(s) for changes"),
            arg!(-m --minify "Minifies the output file(s)"),
            arg!(-i --init "Creates a json file with specific settings"),
            arg!(-u --unused "Clean up unsued variables and style declarations"),
            arg!(-s --search "Searches the program for core features")
                .value_parser(*&SEARCH_PARAM)
                .action(ArgAction::Set),
            arg!(-d --dev "Opens a new browser window, displaying all current activities"),
        ])
        .styles(display_styles)
        .arg_required_else_help(true) // exp
        .get_matches();

    matches
}

// The second task of this project will be the implementation of useful filesystem functions
