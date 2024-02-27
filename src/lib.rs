#![allow(dead_code)]
const VERSION: &str = "mano v0.1.0";

/// The first task of this project will be the creation of a robust command line interface.
 pub mod mano_cli {
     use::clap::Parser;

     #[derive(Parser, Debug)]
     #[command(version, about, long_about = None)]

     pub struct Args {
         #[arg(short, long)]
         name: String,
     }

     pub fn cli() {

     }
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
