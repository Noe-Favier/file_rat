use std::path::PathBuf;

use crate::structs::enums::compression_type::CompressionType;
use crate::structs::rat_meta::{RatMeta, RatMetaObject};
use structs::rat_file::RatFile;

mod structs;

fn main() {
    //let rfile= new Rfile<Metadata.class>(fileRef);

    /*
    > add to end of data -> binrw file content
    > add to end of hadr -> (start>end + file name + metadata in json + unique id)


    file format :

    [--@DATA@--|headers]


    pre-cond to an update :
        - the disk needs filesize in worst case of room + headers
        - rights 700 on file
        - rat file is not in EOF

    technical specs :
        - the Metadata motherclass must have a serial id autocalculated
        - the rat processor can be made
                - serial ignorant to disable serial checks
                - high/low compression level
                - encrypted headers


    */

    let mut rat_file: RatFile<RatMeta<RatMetaObject>> =
        structs::rat_file::RatFile::new(PathBuf::from("./test.rat"), true, CompressionType::Best)
            .unwrap();
    println!("{:?}", rat_file.get_item_header_section_index());

    let _f1 = rat_file
        .insert_to_rat_file(PathBuf::from("./1.txt"), build_meta("alice", "invoice", 1))
        .expect("Error inserting file to rat file");
    println!(
        "HEADER INDEX OF 1.txt {:?}",
        rat_file.get_item_header_index(0)
    );

    let f2 = rat_file
        .insert_to_rat_file(PathBuf::from("./2.txt"), build_meta("bob", "report", 2))
        .expect("Error inserting file to rat file");
    println!(
        "HEADER INDEX OF 2.txt {:?}",
        rat_file.get_item_header_index(1)
    );

    let f3 = rat_file
        .insert_to_rat_file(PathBuf::from("./3.txt"), build_meta("carol", "archive", 3))
        .expect("Error inserting file to rat file");

    println!(
        "HEADER INDEX OF 1.txt {:?}",
        rat_file.get_item_header_index(0)
    );

    println!(
        "HEADER INDEX OF 2.txt {:?}",
        rat_file.get_item_header_index(1)
    );

    println!(
        "HEADER INDEX OF 3.txt {:?}",
        rat_file.get_item_header_index(2)
    );

    println!("-----------------------------");

    let f2_extract_dest = PathBuf::from(format!("{}.extracted", f2.name));
    let extracted_2 = rat_file
        .extract(f2.id, f2_extract_dest, false)
        .expect("Error extracting file 2 from rat file");
    println!("Extracted file 2 to {:?} (no remove)", extracted_2);

    print_rat_file_size_in_bytes(&rat_file).unwrap();

    let f3_extract_dest = PathBuf::from(format!("{}.extracted", f3.name));
    let extracted_3 = rat_file
        .extract(f3.id, f3_extract_dest, true)
        .expect("Error extracting file 3 from rat file");
    println!(
        "Extracted file 3 to {:?} and removed from archive",
        extracted_3
    );
    print_rat_file_size_in_bytes(&rat_file).unwrap();

    println!("-----------------------------");

    println!("+ Current files in rat file:");
    println!("{:?}", rat_file.list_rat_file().unwrap());

    println!("-----------------------------");

    rat_file
        .remove(f2.id)
        .expect("Error removing file 2 from rat file");
    println!("Removed file 2 from rat file");

    print_rat_file_size_in_bytes(&rat_file).unwrap();

    println!("-----------------------------");
    println!("+ Current files in rat file:");
    println!("{:?}", rat_file.list_rat_file().unwrap());
}

fn print_rat_file_size_in_bytes<T>(rat_file: &RatFile<T>) -> std::io::Result<()> {
    let metadata = std::fs::metadata(&rat_file.file_path)?;
    println!("Rat file size: {} bytes", metadata.len());
    Ok(())
}

fn build_meta(owner: &str, category: &str, priority: u64) -> RatMeta<RatMetaObject> {
    let mut meta = RatMeta::new_object();
    meta.insert_custom("owner", owner);
    meta.insert_custom("category", category);
    meta.insert_custom("priority", priority);
    meta
}
