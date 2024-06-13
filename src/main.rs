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

    for i in 1..6 + 1 {
        let path = PathBuf::from(format!("./file{}.txt", i));
        file.add_file(&path).expect("Failed to add file");
    }
    println!("\n\nfiles : {:?}\n\n", file.get_file_list().unwrap());

    for x in file.get_file_list().unwrap() {
        let path = PathBuf::from(format!("./{}-RESULT.txt", x.name));
        file
        .extract_file(
            uuid::Uuid::parse_str(x.uuid.to_string().as_str()).expect("cant parse uuid"),
            path,
        ).expect("Failed to extract file");
    }
}
