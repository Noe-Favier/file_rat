use std::path::PathBuf;

use crate::structs::enums::compression_type::CompressionType;
use metatest::MetadataTest;
use structs::{f_item::FileItem, rat_file::RatFile};

mod metatest;
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

    let mut rat_file: RatFile<MetadataTest> =
        structs::rat_file::RatFile::new(PathBuf::from("./test.rat"), true, CompressionType::Best)
            .unwrap();
    println!("{:?}", rat_file.get_item_header_section_index());

    rat_file
        .insert_to_rat_file(PathBuf::from("./1.txt"), MetadataTest::new())
        .expect("Error inserting file to rat file");
    println!(
        "HEADER INDEX OF 1.txt {:?}",
        rat_file.get_item_header_index(0)
    );

    rat_file
        .insert_to_rat_file(PathBuf::from("./2.txt"), MetadataTest::new())
        .expect("Error inserting file to rat file");
    println!(
        "HEADER INDEX OF 2.txt {:?}",
        rat_file.get_item_header_index(1)
    );

    rat_file
        .insert_to_rat_file(PathBuf::from("./3.txt"), MetadataTest::new())
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

    // rat_file
    //     .remove(
    //         (rat_file.list_rat_file().expect("Error getting list") as Vec<FileItem<MetadataTest>>)
    //             .get(1)
    //             .unwrap()
    //             .id,
    //     )
    //     .expect("Error removing file from rat file");

    println!("{:?}", rat_file.list_rat_file().unwrap());
}
