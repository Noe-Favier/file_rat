use std::path::PathBuf;
mod structs;

use {structs::rat_file::RatFile};

fn main() {

    let path = PathBuf::from("./test.rat");

    if RatFile::can_create_at(path.to_owned()) {
        let _file = RatFile::create_at(path).unwrap();
        println!("File created at: {}", _file.path().to_str().unwrap());
    }
    
}
