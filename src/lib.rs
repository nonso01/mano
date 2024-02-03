#![allow(dead_code)]
const VERSION: &str = "mano v0.1.0";

/// The first task of this project will be the creation of a robust command line interface.
 pub mod mano_cli {
     // use std::{self, collections::{HashMap}};

     pub struct CommandOptions<S, E>  {
         pub help: S,
         pub version: S,
         pub init: E,
         pub watch: E,
         pub minify: E,
         pub clean: E,
         pub update: E,
     }

     pub struct CommandOptionStyle {
         // using anstyle
     }

     pub trait DefaultOptions {
         fn display_help(&self) -> String;
     }

     impl<S, E> DefaultOptions for CommandOptions<S, E> {
         fn display_help(&self) -> String {
             format!("{}", String::from("A short help message!"))
         }
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
