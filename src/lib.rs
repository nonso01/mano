#![allow(dead_code)]
const VERSION: &str = "v0.1.0";

// The first task of this project,
// will be the creation of a robust command line interface.

#[allow(unused_imports)]
use ::clap::{
    arg,
    builder::{styling::AnsiColor, Styles},
    Arg, Command,
};

pub fn cli() {
    let display_styles = Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Blue.on_default())
        .literal(AnsiColor::Cyan.on_default());

    let _matches = Command::new("mano.rs")
        .author("Nonso Martin")
        .version(env!("CARGO_PKG_VERSION"))
        .about(
            "\tmano.rs is a standalone software crafted for generating css styles on the go!,\
                    \n\tbuilt with rust for more efficiency and productivity.",
        )
        .args([
            arg!(-w --watch "Watches the target file(s) for changes"),
            arg!(-m --minify "Minifies the output file(s)"),
            arg!(-i --init "Creates a json file with specific settings"),
            arg!(-u --unused "Clean up unsued variables and style declarations"),
            arg!(-s --search "Search for base or default styles"),
            arg!(-t --tab "Opens a new browser window, displaying all curreent activities"),
        ])
        .styles(display_styles)
        .arg_required_else_help(true) // exp
        .get_matches();
}

// The second task of this project will be the implementation of useful filesystem functions
