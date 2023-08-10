use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::{BufReader, Error, ErrorKind, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    usize,
};
extern crate base64;
use super::rfile::RFile;
use base64::{engine::general_purpose, Engine as _, decoded_len_estimate, encoded_len};
use memmap2::MmapMut;
use std::io::ErrorKind::InvalidInput;
use uuid::Uuid;

//create a buffer of 1000 byte
const BUFFER_SIZE: usize = 12000;

#[allow(dead_code)]
pub struct RatFile {
    path: PathBuf,
    pub file: File,
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
        //TODO: wtf optimize this !!!
        let mut rat_file = &self.file;
        let mut file_list: Vec<RFile> = Vec::new();

        let reader = BufReader::new(rat_file);
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
            if file == "" {
                //the last file is empty, because metadata ends with a ;
                continue;
            }
            file_list.push(RFile::deserialize(file.to_string()));
        }

        rat_file.seek(SeekFrom::Start(0))?; //getting back to the start of the rat file to let the other functions work
        Ok(file_list)
    }

    pub fn add_file(&self, file_path: &PathBuf) -> Result<(), Error> {
        let mut rat_file = &self.file;
        let mut file = File::open(file_path)?;
        let mut reader = BufReader::new(rat_file);
        let mut total_byte_written = 0;

        let rat_size = rat_file
            .metadata()
            .unwrap_or_else(|err| {
                panic!("Error getting metadata from file: {}", err);
            })
            .len();

        let rfile: RFile = RFile::new_from(file_path, self);

        rat_file.seek(SeekFrom::Start(0))?; //getting back to start of file since we were at the end
        let pos = reader
            .by_ref()
            .bytes()
            .position(|b| b.unwrap() == b'|')
            .ok_or(InvalidInput)?
            + 1; //+1 because we stop at the | char (and we want to write after it)

        //write file metadata
        rat_file.set_len(rat_size + rfile.serialize().len() as u64)?;
        let mut mmap = unsafe { MmapMut::map_mut(rat_file)? };

        mmap.copy_within(pos..rat_size as usize, pos + rfile.serialize().len()); //moving the file data to the right
        mmap[pos..pos + rfile.serialize().len()].copy_from_slice(rfile.serialize().as_bytes()); //writing the file metadata between
        mmap.flush()?;
        total_byte_written += rfile.serialize().len(); //updating the total byte written

        //write file data
        rat_file.seek(SeekFrom::End(0))?; //getting back to the end of the rat file

        let mut buffer = [0; BUFFER_SIZE];
        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }

            rat_file.write_all(&buffer[0..bytes_read]).and_then(
                |_| {
                    total_byte_written += bytes_read; //updating the total byte written
                    Ok(())
                },
            )?; //writing the buffer to the rat file
        }

        rat_file.seek(SeekFrom::Start(0))?; //getting back to the start of the rat file to let the other functions work
        Ok(())
    }

    pub fn extract_file(&self, uuid: Uuid, dest: PathBuf) -> Result<(), Error> {
        let mut rat_file = &self.file;
        let file = self.get_rfile_by_uuid(uuid)?;

        rat_file.seek(SeekFrom::Start(file.byte_start - 1)).unwrap();

        let mut destination = File::create(dest)?;
        destination.set_len(file.size)?;
        let mut buffer = [0; BUFFER_SIZE];
        let mut remaining_bytes = file.size;

        while remaining_bytes > 0 {
            let bytes_to_read = std::cmp::min(buffer.len() as u64, remaining_bytes) as usize;
            let bytes_read = rat_file.read(&mut buffer[0..bytes_to_read])?;

            if bytes_read == 0 {
                break;
            }

            destination.write_all(&buffer[0..bytes_read])?;

            remaining_bytes -= bytes_read as u64;
        }

        rat_file.seek(SeekFrom::Start(0))?; //getting back to the start of the rat file to let the other functions work
        Ok(())
    }

    pub fn update_files_index(&self, mut amount: usize, positive: bool) -> Result<(), Error> {
        //Recursively increment if positive, decrement if negative

        let mut rat_file = &self.file;
        let rfiles: Vec<RFile> = self.get_file_list().unwrap();

        for mut f in rfiles {
            let old_meta: String = f.serialize();
            let new_meta: String = f.update_index(amount, positive);
            if new_meta.len() > old_meta.len() {
                self.update_files_index(new_meta.len() - old_meta.len(), true)?;
            }else if new_meta.len() < old_meta.len() {
                self.update_files_index(old_meta.len() - new_meta.len(), false)?;
            }

            //TODO: WRITE CHANGES
        }

        Ok(())
    }

    pub fn find_metadata_start_by_uuid(&self, uuid: Uuid) -> Result<u64, Error> {
        let mut rat_file = &self.file;

        let mut uuid_buffer = [0; 36];

        let found_flag: bool = false;

        rat_file.seek(SeekFrom::Start(0)).unwrap();
        
        let mut char_buffer = [0; 1];
        while !found_flag && (rat_file.stream_position()? < rat_file.metadata()?.len())  {
            rat_file.read(&mut char_buffer)?;
            if (char_buffer[0] == b';') || (char_buffer[0] == b'|') {
                rat_file.read(&mut uuid_buffer)?;
                let file_uuid = Uuid::parse_str(std::str::from_utf8(&uuid_buffer).unwrap()).unwrap();
                if file_uuid == uuid {
                    return Ok(rat_file.stream_position().unwrap() - 37);
            }
                
            }
        }

        return Err(Error::new(ErrorKind::NotFound, "File not found")); 
    }

    pub fn get_rfile_by_uuid(&self, uuid: Uuid) -> Result<RFile, Error> {
        //TODO: test & finish this
        let rfiles: Vec<RFile> = self.get_file_list().unwrap();
        let file: RFile = rfiles
            .iter()
            .find(|&rfile| rfile.uuid == uuid)
            .expect(format!("File with uuid {} not found", uuid).as_str())
            .clone();

        println!("Found : {}", file.name);

        Ok(file)
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

impl std::clone::Clone for RatFile {
    fn clone(&self) -> Self {
        RatFile {
            path: self.path.clone(),
            file: self.file.try_clone().unwrap(),
        }
    }
}
