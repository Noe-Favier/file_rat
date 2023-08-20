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

    for i in 1..3 + 1 {
        let path = PathBuf::from(format!("./{}.txt", i));
        file.add_file(&path).expect("Failed to add file");
    }

    println!("\n\nfiles : {:?}\n\n", file.get_file_list().unwrap());

    // file.extract_file(
    //     uuid::Uuid::parse_str("2a26591d-2a26-11ee-9f2b-010203040506").expect("cant parse uuid"),
    //     PathBuf::from("./result.txt"),
    // )
    // .expect("Failed to extract file");
}
