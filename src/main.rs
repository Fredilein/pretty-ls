#[macro_use]
extern crate clap;

use std::path::{Path};

use clap::App;

mod core;


fn main() -> std::io::Result<()> {
    let possible_args = load_yaml!("args.yml");
    let matches = App::from_yaml(possible_args).get_matches();

    let input_path = Path::new(matches.value_of("PATH").unwrap());
    let path = input_path.canonicalize().unwrap();

    println!("Listing directory {}", path.display());
    core::ls(&path);
    Ok(())
}

