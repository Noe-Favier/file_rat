use std::{
    fs::File,
    io::{BufReader, Error, ErrorKind, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use super::rfile::RFile;
use std::io::ErrorKind::InvalidInput;
use uuid::Uuid;

#[allow(dead_code)]
pub struct RatFile {
    path: PathBuf,
    file: File,
}

#[allow(dead_code)]
impl RatFile {
    pub fn new_from(path: &PathBuf) -> Result<RatFile, Error> {
        if Path::new(path.as_path()).exists() {
            //TODO: check if file is a rat file
            //TODO: read metadata and inject in struct fields
            return Ok(RatFile {
                path: path.clone(),
                file: File::open(path)?,
            });
        } else {
            return Err(Error::new(ErrorKind::NotFound, "File not found"));
        }
    }

    pub fn create_at(path: &PathBuf) -> Result<RatFile, Error> {
        if Path::new(&path).exists() {
            return Err(Error::new(ErrorKind::AlreadyExists, "File already exists"));
        } else {
            let mut new_rat_file = File::create(&path)?;

            new_rat_file.write_all(b"0;1;0|/")?;

            return Ok(RatFile {
                path: path.to_owned(),
                file: new_rat_file,
            });
        }
    }

    pub fn can_open_at(path: &PathBuf) -> bool {
        if Path::new(&path).exists() {
            //TODO: check if file is a rat file
            return true;
        } else {
            return false;
        }
    }

    pub fn can_create_at(path: &PathBuf) -> bool {
        if Path::new(&path).exists() {
            return false;
        } else {
            return true;
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

#[allow(dead_code)]
impl RatFile {
    pub fn get_file_list(&self) -> Result<Vec<RFile>, Error> {
        let mut file_list: Vec<RFile> = Vec::new();

        let reader = BufReader::new(&self.file);
        let mut gather_flag = false;
        let mut file_list_data: Vec<u8> = Vec::new();
        for byte in reader.bytes() {
            let b = byte?;
            if b == b'|' {
                //we are getting to file list, we start gathering file list
                gather_flag = true;
            } else if b == b'/' {
                //we are getting to file data, we finished gathering file list
                break;
            } else if gather_flag {
                file_list_data.push(b);
            }
        }

        let file_list_data = String::from_utf8(file_list_data).unwrap_or(String::new());

        for file in file_list_data.split(';') {
            let mut file_data = file.split(',');
            //-----

            let file_uuid = Uuid::parse_str(file_data.next().unwrap()).unwrap();
            let file_name = file_data.next().unwrap_or("unamed");

            let byte_start = file_data.next().unwrap_or("0").parse::<u64>().unwrap();
            let byte_end = file_data.next().unwrap_or("0").parse::<u64>().unwrap();

            let is_dir = file_data.next().unwrap_or("0");
            let is_file = file_data.next().unwrap_or("0");

            file_list.push(RFile::new(
                file_uuid,
                file_name.to_string(),
                byte_start - byte_end,
                byte_start,
                byte_end,
                is_dir == "1",
                is_file == "1",
            ));
        }

        Ok(file_list)
    }

    pub fn add_file(&self, file_path: &PathBuf) -> Result<(), Error> {
        let mut rat_file = &self.file;
        let mut file = File::open(file_path)?;
        let mut buffer = Vec::new();

        let start = rat_file.seek(std::io::SeekFrom::End(-1))?; //seek to end of file

        file.read_to_end(&mut buffer)?;
        rat_file.write_all(&buffer)?;
        // Seek to the position of the first occurrence of the character '|' where we will start writing the file list
        let mut reader = BufReader::new(rat_file);

        rat_file.seek(SeekFrom::Start(0))?; //getting back to start of file since we were at the end
        let pos = reader
            .by_ref()
            .bytes()
            .inspect(|b| {
                let x = b.as_ref().unwrap();
                println!("Read byte: {:?}", String::from_utf8(vec![*x]));
            })
            .position(|b| b.unwrap() == b'|')
            .ok_or(InvalidInput)?;

        file.seek(SeekFrom::Start(pos as u64))?; //getting to the position of the first occurrence of the character '|'

        let rfile: RFile = RFile::new_from(file_path, start);

        rat_file.write_all(rfile.serialize().as_bytes())?;
        rat_file.flush()?;

        Ok(())
    }
}

impl std::fmt::Display for RatFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RatFile {{ path: {} }}", self.path.display())
    }
}

impl std::fmt::Debug for RatFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RatFile {{ path: {} }}", self.path.display())
    }
}
