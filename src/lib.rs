#![allow(dead_code)]
const VERSION: &str = "mano v0.1.0";

/// The first task of this project will be the creation of a robust command line interface.
 pub mod mano_cli {
     use std::{self, env};

     pub struct CommandLineOptions  { // what about lifetimes ?
         pub help: String,
         pub version: String,
         pub init: bool,
         pub watch: bool,
         pub minify: bool,
         pub clean: bool,
         pub update: bool,
     }

     pub struct CommandLineOptionStyle {
         // using anstyle
     }

     pub trait CommandLineOperations {
         fn display_help(&self) -> String;
     }

     impl CommandLineOperations for CommandLineOptions {
         fn display_help(&self) -> String {
             format!("{}", String::from("A short help message!"))
         }
     }

     pub fn process_args() -> Vec<String> {
         let mut _args: Vec<String> = env::args().collect();
         dbg!(&mut _args);
         _args
     }

     pub fn cli() {}
 }

/// The second of this project will be the implementation of useful filesystem functions
 pub mod mano_fs {
     pub fn create() {}

     pub fn create_dir() {}

     pub fn read() {}

     pub fn read_dir() {}

     pub fn write() {}

     pub fn delete() {}
 }
