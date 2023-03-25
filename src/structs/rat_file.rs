use std::{
    fs::File,
    io::{Error, ErrorKind, Write},
    path::{Path, PathBuf},
};

#[allow(dead_code)]
pub struct RatFile {
    path: PathBuf,

    //std metadata
    nb_file: u32,
}

#[allow(dead_code)]
impl RatFile {
    pub fn new_from(path: PathBuf) -> Result<RatFile, Error> {
        if Path::new(&path).exists() {
            //TODO: check if file is a rat file
            //TODO: read metadata
            return Ok(RatFile {
                path: path,
                nb_file: 0,
            });
        } else {
            return Err(Error::new(ErrorKind::NotFound, "File not found"));
        }
    }

    pub fn create_at(path: PathBuf) -> Result<RatFile, Error> {
        if Path::new(&path).exists() {
            return Err(Error::new(ErrorKind::AlreadyExists, "File already exists"));
        } else {
            let mut new_rat_file = File::create(&path)?;

            new_rat_file.write_all(b"1;0;0//")?;

            return Ok(RatFile {
                path: path,
                nb_file: 0,
            });
        }
    }

    pub fn can_open_at(path: PathBuf) -> bool {
        if Path::new(&path).exists() {
            //TODO: check if file is a rat file
            return true;
        } else {
            return false;
        }
    }

    pub fn can_create_at(path: PathBuf) -> bool {
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
