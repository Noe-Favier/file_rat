use crate::structs::f_item::FileItem;
use std::{
    fs::File,
    io::{Error, ErrorKind, Read, Seek, SeekFrom, Write},
    path::PathBuf,
};

#[allow(dead_code)]
pub struct RatFile<T> {
    pub(crate) files: Vec<FileItem<T>>,

    pub(crate) file_path: PathBuf,
    pub(crate) file_size: u64,
}

impl<T> RatFile<T> {
    pub(crate) const BUFFER_SIZE: usize = 2000;
    pub(crate) const BUFFER_SIZE_HEADERS: usize = 1024;
    pub(crate) const HEADER_SEPARATOR: u8 = b'|';
    const BASE_RAT_FILE_CONTENT: &'static [u8] = b"|;";


    pub fn new(file_path: PathBuf, can_create: bool) -> Result<Self, Error> {

        if !file_path.exists() && !can_create {
          return Err(Error::new(ErrorKind::NotFound, "File not found"))
        }else if !file_path.exists() && can_create{
            let mut file = File::create(&file_path)?;
            file.write(Self::BASE_RAT_FILE_CONTENT)?;
        }

        Ok(Self {
            files: Vec::new(),
            file_path,
            file_size: 0,
        })
    }
    
    pub(crate) fn get_header_start(rat_file_descriptor: &mut File) -> Result<u64, Error> {
        let current_position: u64 = rat_file_descriptor.stream_position()?;
    
        let buffer_size = Self::BUFFER_SIZE_HEADERS;
        let mut buffer = vec![0; buffer_size];
        let mut position = rat_file_descriptor.seek(SeekFrom::End(0))?;
        
        while position > 0 {
            let read_size = if position < buffer_size as u64 {
                position as usize
            } else {
                buffer_size
            };
    
            position = position.saturating_sub(read_size as u64);
            rat_file_descriptor.seek(SeekFrom::Start(position))?;
            let bytes_read = rat_file_descriptor.read(&mut buffer[..read_size])?;
    
            if bytes_read == 0 {
                break;
            }
    
            if let Some(pos) = buffer[..bytes_read].iter().rposition(|&x| x == Self::HEADER_SEPARATOR) {
                let header_start = (position + pos as u64) + 1; // +1 to skip the separator
                rat_file_descriptor.seek(SeekFrom::Start(current_position))?;
                return Ok(header_start);
            }
        }
    
        rat_file_descriptor.seek(SeekFrom::Start(current_position))?;
        Err(Error::new(ErrorKind::Other, "Header not found"))
    }
    
}