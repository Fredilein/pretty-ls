use std::path::{PathBuf};
use std::fs;

use crate::entry::{self, Entry};


pub fn ls(path: &PathBuf) {
    if path.is_dir() {
        let contents = fs::read_dir(path).unwrap();
        // for e in contents {
        //     entry::print(e.unwrap());
        // }
        entry::print_all(contents);
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


