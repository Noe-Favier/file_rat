use std::{borrow::BorrowMut, fs::{File, Metadata}, io::Empty, path::PathBuf};

use metatest::MetadataTest;
use structs::rat_file::RatFile;
use crate::structs::enums::compression_type::CompressionType;

mod structs;
mod metatest;

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

    let mut rat_file: RatFile<MetadataTest> = structs::rat_file::RatFile::new(PathBuf::from("./test.rat"), true, CompressionType::Best).unwrap();
    println!("{:?}", rat_file.get_item_header_index());

    rat_file.insert_to_rat_file(PathBuf::from("./1.txt"), MetadataTest::new()).expect("Error inserting file to rat file");
}
