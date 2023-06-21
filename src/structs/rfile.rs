use std::{ffi::OsStr, path::PathBuf, fmt::{Debug, Display}};

use uuid::{Context, Timestamp, Uuid};

use super::rat_file::RatFile;

pub struct RFile {
    pub uuid: Uuid,

    pub size: u64,
    pub name: String,

    byte_start: u64,
}

#[allow(dead_code)]
impl RFile {
    pub fn new(uuid: Uuid, name: String, size: u64, byte_start: u64) -> RFile {
        RFile {
            uuid,
            name,
            byte_start,
            size,
        }
    }

    pub fn new_from(file_path: &PathBuf, rat_file: &RatFile) -> RFile {
        let context = Context::new(rand::random::<u16>());
        let ts = Timestamp::now(context);
        let uuid = Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6]);
        let name: String = file_path
            .file_name()
            .unwrap_or(&OsStr::new("unamed"))
            .to_str()
            .unwrap_or("unamed")
            .to_string();
        let size = file_path.metadata().unwrap().len();


        /*
        the byte start needs an offset generated by the metadata writing :
            - the size of the rat file
            - the size of the uuid
            - the size of the name
            - the size of the size (b64)
            - 3 commas (separating metadata)
        */
        let metadata_size = 36 //uuid
        + name.len() as u64 //name
        + RFile::get_size_in_b64(size).to_string().len() as u64 //size (b64)
        + 3 //commas
        + 1 //metadata_separator (;)
        + 1 //offset 
        ;
        
        let mut byte_start = rat_file.file.metadata().unwrap().len() + metadata_size;
        byte_start = byte_start + byte_start.to_string().len() as u64; //add the size of the byte start (b64)
        
        RFile::new(uuid, name, size, byte_start)
    }

    pub(crate) fn set_byte_start(&mut self, byte_start: u64) {
        self.byte_start = byte_start;
    }

    pub fn serialize(&self) -> String {
        let mut data = String::new();
        data.push_str(&self.uuid.to_string());
        data.push_str(",");
        data.push_str(&self.name);
        data.push_str(",");
        data.push_str(&self.byte_start.to_string());
        data.push_str(",");
        data.push_str(RFile::get_size_in_b64(self.size).to_string().as_str()); //size (b64)
        data.push_str(";");
        data
    }

    pub fn deserialize(serialized_rfile: String) -> RFile {
        
            let mut file_data = serialized_rfile.split(',');
            //-----

            let file_uuid = Uuid::parse_str(file_data.next().unwrap()).unwrap();
            let file_name = file_data.next().unwrap_or("unamed");
            let byte_start = file_data.next().unwrap_or("0").parse::<u64>().unwrap();
            let size = file_data.next().unwrap_or("0").parse::<u64>().unwrap();

            RFile::new(
                file_uuid,
                file_name.to_string(),
                byte_start,
                size,
            )
    }

    fn get_size_in_b64(original_size: u64) -> u64 {
        let x: f64 = f64::ceil(original_size as f64 / 3.0);
        (x * 4.0) as u64
    }
}

impl Debug for RFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RFile")
            .field("uuid", &self.uuid)
            .field("size", &self.size)
            .field("name", &self.name)
            .field("byte_start", &self.byte_start)
            .finish()
    }
}

impl Display for RFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}