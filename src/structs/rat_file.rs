use crate::structs::f_item::FileItem;
use crate::structs::enums::compression_type::CompressionType;
use std::{
    fs::File,
    io::{Empty, Error, ErrorKind, Read, Seek, SeekFrom, Write},
    path::PathBuf,
};

#[allow(dead_code)]
pub struct RatFile<T> {
    pub(crate) files: Vec<FileItem<T>>,

    pub(crate) file_path: PathBuf,
    pub(crate) file_size: u64,

    pub(crate) compression_type: CompressionType,
}

#[allow(dead_code)]
impl<T> RatFile<T> {
    pub(super) const RAT_VERSION: u8 = b'1';

    pub(crate) const BUFFER_SIZE: usize = 2000;
    pub(crate) const BUFFER_SIZE_HEADERS: usize = 1024;
    pub(crate) const HEADER_SEPARATOR: u8 = b'|';
    pub(crate) const HEADER_ITEM_SEPARATOR: u8 = b';';
    pub(crate) const HEADER_GENERAL_SEPARATOR: u8 = b'/';

    pub fn new(
        file_path: PathBuf, 
        can_create: bool,
        compression_type: CompressionType,
        ) -> Result<Self, Error> {

        if !file_path.exists() && !can_create {
          return Err(Error::new(ErrorKind::NotFound, "File not found"))
        }else if !file_path.exists() && can_create{

            let base_content: [u8; 7] = [
                //"|" declaring the start of the global header section
                RatFile::<Empty>::HEADER_SEPARATOR, 
                //version of the rat file
                RatFile::<Empty>::RAT_VERSION, 

                // ";"
                RatFile::<Empty>::HEADER_ITEM_SEPARATOR, 

                //flag declaring the compression level of the file
                (compression_type.clone() as isize).to_string().as_bytes()[0],

                // ";"
                RatFile::<Empty>::HEADER_ITEM_SEPARATOR, 

                //"0" lock flag
                b'0',

                //"/" declaring the start of the item header section
                RatFile::<Empty>::HEADER_GENERAL_SEPARATOR,
            ];

            let mut file = File::create(&file_path)?;
            file.write(&base_content)?;
        }

        Ok(Self {
            files: Vec::new(),
            file_path,
            file_size: 0,
            compression_type: compression_type,
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