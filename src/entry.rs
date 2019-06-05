use std::path::Path;
use std::fs::DirEntry;
use std::ffi::OsStr;


pub fn print(entry: DirEntry) {
    let file_name = entry.file_name();
    let name = file_name.to_str().unwrap();
    let metadata = entry.metadata().unwrap();
    let symbol;
    if metadata.is_dir() {
        symbol = get_dir_symbol(name);
    } else if metadata.is_file() {
        symbol = get_file_symbol(name);
    } else {
        symbol = "\u{f15b}".chars().next().unwrap();
    }
    let len = metadata.len();

    println!("{}  {}", symbol, name);
}


fn get_dir_symbol(file_name: &str) -> char {
    let sym = match file_name {
      ".git" => "\u{f1d3}",
      _      => "\u{f115}",
    };
    sym.chars().next().unwrap()
}

fn get_file_symbol(file_name: &str) -> char {
    // If file has no extension, ext gets set to "default"
    let ext_option = Path::new(file_name)
                            .extension()
                            .and_then(OsStr::to_str);

    let ext = match ext_option {
        Some(e) => e,
        None    => "default",
    };

    let sym = match ext {
      "android"   => "\u{e70e}",
      "apple"     => "\u{f179}",
      "audio"     => "\u{f001}",
      "avro"      => "\u{e60b}",
      "c"         => "\u{e61e}",
      "clj"       => "\u{e768}",
      "coffee"    => "\u{f0f4}",
      "conf"      => "\u{e615}",
      "cpp"       => "\u{e61d}",
      "css"       => "\u{e749}",
      "d"         => "\u{e7af}",
      "dart"      => "\u{e798}",
      "db"        => "\u{f1c0}",
      "diff"      => "\u{f440}",
      "doc"       => "\u{f1c2}",
      "docker"    => "\u{f308}",
      "ebook"     => "\u{e28b}",
      "env"       => "\u{f462}",
      "epub"      => "\u{e28a}",
      "erl"       => "\u{e7b1}",
      "font"      => "\u{f031}",
      "gform"     => "\u{f298}",
      "git"       => "\u{f1d3}",
      "go"        => "\u{e626}",
      "html"      => "\u{f13b}",
      "image"     => "\u{f1c5}",
      "iml"       => "\u{e7b5}",
      "java"      => "\u{e204}",
      "js"        => "\u{e74e}",
      "json"      => "\u{e60b}",
      "jsx"       => "\u{e7ba}",
      "less"      => "\u{e758}",
      "log"       => "\u{f18d}",
      "lua"       => "\u{e620}",
      "md"        => "\u{f48a}",
      "mustache"  => "\u{e60f}",
      "npmignore" => "\u{e71e}",
      "pdf"       => "\u{f1c1}",
      "php"       => "\u{e73d}",
      "pl"        => "\u{e769}",
      "ppt"       => "\u{f1c4}",
      "psd"       => "\u{e7b8}",
      "py"        => "\u{e606}",
      "r"         => "\u{f25d}",
      "rb"        => "\u{e21e}",
      "rdb"       => "\u{e76d}",
      "rss"       => "\u{f09e}",
      "rubydoc"   => "\u{e73b}",
      "sass"      => "\u{e603}",
      "scala"     => "\u{e737}",
      "shell"     => "\u{f489}",
      "sh"        => "\u{f489}",
      "sqlite3"   => "\u{e7c4}",
      "styl"      => "\u{e600}",
      "tex"       => "\u{e600}",
      "ts"        => "\u{e628}",
      "twig"      => "\u{e61c}",
      "txt"       => "\u{f15c}",
      "video"     => "\u{f03d}",
      "vim"       => "\u{e62b}",
      "windows"   => "\u{f17a}",
      "xls"       => "\u{f1c3}",
      "xml"       => "\u{e619}",
      "yml"       => "\u{f481}",
      "zip"       => "\u{f15b}",
      _           => "\u{f15b}",
    };
    sym.chars().next().unwrap()
}
