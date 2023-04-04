use std::path::PathBuf;
mod structs;

use structs::rat_file::RatFile;

fn main() {
    let path = PathBuf::from("./test.rat");

    if RatFile::can_create_at(&path) {
        let _file = RatFile::create_at(&path).unwrap();
        println!("File created at: {}", _file.path().to_str().unwrap());
    }

    if !RatFile::can_open_at(&path) {
        panic!("File cannot be opened");
    }
    let file = RatFile::new_from(&path).unwrap();
    println!("File opened at: {}", file.path().to_str().unwrap());

    let helloworld_path = PathBuf::from("./helloworld.txt");
    file.add_file(&helloworld_path).expect("Failed to add file");
}
