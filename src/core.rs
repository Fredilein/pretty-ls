use std::path::{PathBuf};
use std::fs;

use crate::entry;


pub fn ls(path: &PathBuf) {
    if path.is_dir() {
        let contents = fs::read_dir(path).unwrap();
        entry::print_all(contents);
    } else {
        println!("Not a directory!");
    }
}
