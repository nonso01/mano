#![allow(dead_code)]
const VERSION: &str = "v0.1.0";

/// The first task of this project will be the creation of a robust command line interface.
 pub mod mano_cli {
     use::clap::{Command, arg,};
     
     pub fn cli() {

         // let display_styles =  
        
         let _matches = Command::new("mano.rs")
             .author("Nonso Martin, https://github.com/nonso01")
             .version(env!("CARGO_PKG_VERSION"))
             .about("mano.rs is a standalone software crafted for generating styles on the go!, similar to tailwindcss, but more powerful in every aspect.")
             .args([
                   arg!(-w --watch "watches the target file(s) for changes"),
                   arg!(-m --minify "minifies the output file(s)"),
                   arg!(-i --init "creates a json file with specified settings")
             ])
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
