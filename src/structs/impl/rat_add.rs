use crate::structs::rat_file::RatFile;
use std::fs::File;

impl<T> RatFile<T> {
    pub(crate) fn insert_to_rat_file(&mut self, file: File) {
        let file_size = file.metadata().unwrap().len();
        let mut buffer = [0; Self::BUFFER_SIZE];

        let mut start = 0;
        let mut end = 0;


    }
}