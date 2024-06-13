//TODO: removed useless seeks at end of each fn
use std::{
    fs::{File},
    path::{PathBuf},
};
extern crate base64;

//create a buffer of 1000 byte
const BUFFER_SIZE: usize = 12000;

#[allow(dead_code)]
pub struct RatFile {
    path: PathBuf,
    pub file: File,
}