#![allow(dead_code)]
use mano::cli;

const CONFIG_FILLE: &str = "mano.json";

fn main() {
    // (1) work on the command line
    let _cmd = cli();

    // we initialize our project
    let __init__ = match _cmd.get_one::<bool>("init") {
        Some(initialize) => initialize,
        _ => todo!(),
    };

    println!("{__init__}");
}
