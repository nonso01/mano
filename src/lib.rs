#![allow(dead_code)]
const VERSION: &str = "v0.1.0";

/// The first task of this project will be the creation of a robust command line interface.
#[allow(unused_imports)]
pub mod mano_cli {
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

        let _cmd = Command::new("mano.rs")
             .author("Nonso Martin, https://github.com/nonso01")
             .version(env!("CARGO_PKG_VERSION"))
             .about("\tmano.rs is a standalone software crafted for generating css styles on the go!,\
                    \n\tbuilt with rust for more efficiency and productivity.")
             .args([
                   arg!(-w --watch "watches the target file(s) for changes"),
                   arg!(-m --minify "minifies the output file(s)"),
                   arg!(-i --init "creates a json file with specified settings"),
                   arg!(-u --unused "clean up unsued variables and style declarations"),
                   arg!(-s --screen <NEWTAB> "open a new browser tab, to display all the activities going on,\n on the current working session")
                  ])
             .styles(display_styles)
             .arg_required_else_help(true) // exp

             .get_matches();
    }
}

/// The second task of this project will be the implementation of useful filesystem functions
pub mod mano_fs {
    pub fn create() {}

    pub fn create_dir() {}

    pub fn read() {}

    pub fn read_dir() {}

    pub fn write() {}

    pub fn delete() {}
}
