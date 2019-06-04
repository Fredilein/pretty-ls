use std::path::{Path, PathBuf};
use std::fs::{self, DirEntry};
use std::ffi::OsStr;

use crate::entry::Entry;


pub fn ls(path: &PathBuf) {
    if path.is_dir() {
        let contents = fs::read_dir(path).unwrap();
        for e in contents {
            let entry = Entry::new(e.unwrap()).unwrap();
            entry.print();
            // println!("{:?}, {:?}", entry.metadata().unwrap(), entry.file_name());
        }
    } else {
        println!("Not a directory!");
    }
}

// fn print_entry(entry: DirEntry) {
//     let metadata = entry.metadata().unwrap();
//     let file_name = entry.file_name();
//     let  file_name_str = file_name.to_str().unwrap();
//     println!("Metadata:\n {:?}", metadata);
//     if metadata.is_dir() {
//         let symbol = get_dir_symbol(file_name.to_str().unwrap());
//         println!("{}  {}", symbol, file_name_str);
//     } else if metadata.is_file() {
//         let symbol = get_file_symbol(file_name.to_str().unwrap());
//         println!("{}  {}", symbol, file_name_str);
//     } else {
//         println!("unknown filetype.");
//     }
// }


