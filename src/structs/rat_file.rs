extern crate base64;

use std::path::PathBuf;
use crate::structs::f_item::FileItem;



#[allow(dead_code)]
pub struct RatFile<T> {
    files: Vec<FileItem<T>>,

    file_path: PathBuf,
    file_size: u64,
}

impl<T> RatFile<T> {
    pub(crate) const BUFFER_SIZE: usize = 12000; //1000 bytes
}