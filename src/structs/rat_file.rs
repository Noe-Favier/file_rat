//TODO: removed useless seeks at end of each fn
use std::{
    borrow::BorrowMut,
    char,
    fs::{create_dir_all, File, OpenOptions},
    io::{BufReader, Error, ErrorKind, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    usize,
};
extern crate base64;
use super::rfile::RFile;
use base64::{decoded_len_estimate, encoded_len, engine::general_purpose, Engine as _};
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
    pub fn get_file_list_until(&self, uuid: Uuid) -> Result<Vec<RFile>, Error> {
        return self.get_file_list().map(|file_list| {
            let mut new_file_list: Vec<RFile> = Vec::new();
            for file in file_list {
                if file.uuid == uuid {
                    break;
                }
                new_file_list.push(file);
            }
            new_file_list
        });
    }

    pub fn get_file_list(&self) -> Result<Vec<RFile>, Error> {
        let mut byte_read: usize = 0;
        let mut char_buffer: [u8; 1] = [0; 1];
        let mut rat_file = &self.file;
        let mut file_list: Vec<RFile> = Vec::new();
        let mut metadata_buffer: Vec<u8> = Vec::new();

        rat_file.flush()?; //flushing the file to be sure changes are written
        rat_file.seek(SeekFrom::Start(0))?; //getting back to start of file

        loop {
            //start of metadata detected
            byte_read = rat_file.read(&mut char_buffer)?;
            print!("{}", char_buffer[0] as char);
            if char_buffer[0] == b'|' {break;}
            else if byte_read == 0 {Err(Error::new(ErrorKind::UnexpectedEof, "Unexpected EOF"))?;}
        }
        loop {
            byte_read = rat_file.read(&mut char_buffer)?;
            print!("{}", char_buffer[0] as char);
            if byte_read == 0 {Err(Error::new(ErrorKind::UnexpectedEof, "Unexpected EOF"))?;}

            if char_buffer[0] == b';' {
                //end of metadata detected
                let metadata: String = String::from_utf8(metadata_buffer.clone()).unwrap();
                let rfile: RFile = RFile::deserialize(metadata);
                file_list.push(rfile);
                metadata_buffer.clear();
            } else if char_buffer[0] == b'/' {
                break;
            } else {
                metadata_buffer.push(char_buffer[0]);
            }
        }

        Ok(file_list)
    }

    pub fn add_file(&self, file_path: &PathBuf) -> Result<(), Error> {
        //region variables
        let mut rat_file = &self.file;
        let mut file = File::open(file_path)?;
        let mut reader = BufReader::new(rat_file);
        let mut total_byte_written = 0;
        //endregion variables

        //region metadata_writing
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
            .position(|b| b.unwrap() == b'/')
            .ok_or(InvalidInput)?
            + 0; //+1 because we stop at the | char (and we want to write after it)

        //write file metadata
        rat_file.set_len(rat_size + rfile.serialize().len() as u64)?;
        let mut mmap = unsafe { MmapMut::map_mut(rat_file)? };

        mmap.copy_within(pos..rat_size as usize, pos + rfile.serialize().len()); //moving the file data to the right
        mmap[pos..pos + rfile.serialize().len()].copy_from_slice(rfile.serialize().as_bytes()); //writing the file metadata between
        mmap.flush()?;
        total_byte_written += rfile.serialize().len(); //updating the total byte written
                                                       //endregion metadata_writing

        //region metadata re-indexing to predecessors
        rat_file.flush()?;
        self.update_predecessors_metadata_indexes(rfile.uuid, total_byte_written, true)?;
        //endregion metadata re-indexing to predecessors

        //write file data
        rat_file.seek(SeekFrom::End(0))?; //getting back to the end of the rat file

        let mut buffer = [0; BUFFER_SIZE];
        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }

            rat_file.write_all(&buffer[0..bytes_read]).and_then(|_| {
                total_byte_written += bytes_read; //updating the total byte written
                Ok(())
            })?; //writing the buffer to the rat file
        }

        rat_file.seek(SeekFrom::Start(0))?; //getting back to the start of the rat file to let the other functions work
        Ok(())
    }

    fn update_predecessors_metadata_indexes( //fixme: this function is not working properly (bad writing location + amount calculation)
        &self,
        uuid: Uuid,
        amount: usize,
        positive: bool,
    ) -> Result<(), Error> {
        //Recursively increment if positive, decrement if negative

        let rat_file: &File = &self.file;
        let rat_file_len = rat_file.metadata()?.len();
        let rfiles: Vec<RFile> = self.get_file_list_until(uuid).unwrap();
        println!("Updating metadata indexes of {} files", rfiles.len());
        for mut f in rfiles {
            println!(
                "Updating metadata of {} with positive {} and amount of {}",
                f.name, positive, amount
            );
            let pos: usize = self.find_metadata_start_by_uuid(f.uuid)? as usize;
            let old_meta: String = f.serialize();
            let new_meta: String = f.update_index(amount, positive);
            let mut size_changed_flag: bool = true;
            //if those changes have generated a new metadata size, we need to update the byte start of the predecessors again
            //TODO: condition not that pretty but it works
            if new_meta.len() > old_meta.len() {
                self.update_predecessors_metadata_indexes(
                    f.uuid,
                    new_meta.len() - old_meta.len(),
                    true,
                )?;
                rat_file.set_len(rat_file_len + (new_meta.len() - old_meta.len()) as u64)?;
            } else if new_meta.len() < old_meta.len() {
                self.update_predecessors_metadata_indexes(
                    f.uuid,
                    old_meta.len() - new_meta.len(),
                    false,
                )?;
                rat_file.set_len(rat_file_len + (old_meta.len() - new_meta.len()) as u64)?;
            } else {
                size_changed_flag = false;
            }

            //TODO: WRITE CHANGES
            let mut mmap = unsafe { MmapMut::map_mut(rat_file)? };
            if size_changed_flag {
                mmap.copy_within(
                    (pos + old_meta.len())..rat_file_len as usize,
                    pos + new_meta.len(),
                ); //moving the next metadata to the right
            }
            mmap[pos..pos + new_meta.len()].copy_from_slice(new_meta.as_bytes()); //writing the file metadata between
            mmap.flush()?;
        }

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

    pub fn find_metadata_start_by_uuid(&self, uuid: Uuid) -> Result<u64, Error> { //TODO: EOF @see get_files_list
        //precond: file is a rat file
        let mut rat_file = &self.file;
        let mut uuid_buffer = [0; 36];
        let mut pipe_found_flag: bool = false;
        let mut char_buffer = [0; 1];
        rat_file.seek(SeekFrom::Start(0)).unwrap();

        loop {
            rat_file.read(&mut char_buffer)?;
            print!("{}", char_buffer[0] as char);
            if char_buffer[0] == b'|' {
                //start of metadata detected
                pipe_found_flag = true;
                rat_file.read(&mut uuid_buffer)?;
                let file_uuid =
                    Uuid::parse_str(std::str::from_utf8(&uuid_buffer).unwrap()).unwrap();
                if file_uuid == uuid {
                    return Ok(rat_file.stream_position().unwrap() - 37);
                }
                continue;
            }
            if pipe_found_flag && (char_buffer[0] == b';') {
                rat_file.read(&mut char_buffer)?;
                if char_buffer[0] == b'/' {
                    //end of metadata detected
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        format!("UUID '{}' not found", uuid),
                    ));
                } else {
                    rat_file.seek(SeekFrom::Current(-1))?;
                }

                rat_file.read(&mut uuid_buffer)?;
                let file_uuid =
                    Uuid::parse_str(std::str::from_utf8(&uuid_buffer).unwrap()).unwrap();
                if file_uuid == uuid {
                    return Ok(rat_file.stream_position().unwrap() - 37);
                }
            }
        }
    }

    pub fn get_rfile_by_uuid(&self, uuid: Uuid) -> Result<RFile, Error> {
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
