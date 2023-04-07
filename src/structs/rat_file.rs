use std::{
    fs::{File, OpenOptions},
    io::{BufReader, Error, ErrorKind, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};
use super::rfile::RFile;
use std::io::ErrorKind::InvalidInput;
use uuid::Uuid;
use memmap2::{Mmap, MmapMut};

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

            let rat_file = OpenOptions::new().write(true).read(true).open(path)?;

            return Ok(RatFile {
                path: path.clone(),
                file: rat_file,
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

            file_list.push(RFile::new(
                file_uuid,
                file_name.to_string(),
                byte_start - byte_end,
                byte_start,
                byte_end,
            ));
        }

        Ok(file_list)
    }

    pub fn add_file(&self, file_path: &PathBuf) -> Result<(), Error> {
        let mut rat_file = &self.file;
        let mut file = File::open(file_path)?;
        let mut buffer: Vec<u8> = Vec::new();
        let mut reader = BufReader::new(rat_file);

        let rat_size = rat_file
            .metadata()
            .unwrap_or_else(|err| {
                panic!("Error getting metadata from file: {}", err);
            })
            .len();

        let file_size = file
            .metadata()
            .unwrap_or_else(|err| {
                panic!("Error getting metadata from file: {}", err);
            })
            .len();

        let rfile: RFile = RFile::new_from(file_path, rat_size, rat_size + file_size);


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

        //write file metadata
        println!("{}", rfile.serialize());
        println!("{:?}", rfile.serialize().bytes());
        
        let mut mmap = unsafe { MmapMut::map_mut(rat_file)?  };
        mmap.copy_within(pos.., pos + rfile.serialize().len());

        //write file data
        rat_file.seek(SeekFrom::End(0))?; //getting back to the end of the rat file
        file.read_to_end(&mut buffer)?; //reading the file to the end and storing it in the buffer
        println!("{:?}", String::from_utf8((&buffer).to_owned())); 
        rat_file.write_all(&buffer)?; //writing the buffer to the rat file
        rat_file.flush()?; //flushing the rat file

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
