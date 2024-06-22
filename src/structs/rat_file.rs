use std::path::PathBuf;
use crate::structs::f_item::FileItem;

const BUFFER_SIZE: usize = 12000; // 1000 bytes

#[allow(dead_code)]
pub struct RatFile<T> {
    pub(crate) files: Vec<FileItem<T>>,

    pub(crate) file_path: PathBuf,
    pub(crate) file_size: u64,
}

impl<T> RatFile<T> {
    pub(crate) const BUFFER_SIZE: usize = BUFFER_SIZE; //1000 bytes

    pub fn new(file_path: PathBuf) -> Self {
        Self {
            files: Vec::new(),
            file_path,
            file_size: 0,
        }
    }
}