use std::{ffi::OsStr, fs::File, path::PathBuf};

use uuid::{Context, Timestamp, Uuid};

pub struct RFile {
    pub uuid: Uuid,

    pub size: u64,
    pub name: String,

    byte_start: u64,
    byte_end: u64,
}

#[allow(dead_code)]
impl RFile {
    pub fn new(
        uuid: Uuid,

        name: String,
        size: u64,

        byte_start: u64,
        byte_end: u64,
    ) -> RFile {
        RFile {
            uuid,
            name,
            size,
            byte_start: byte_start,
            byte_end: byte_end,
        }
    }

    pub fn new_from(file_path: &PathBuf, byte_start: u64, byte_end: u64) -> RFile {
        let context = Context::new(rand::random::<u16>());
        let ts = Timestamp::now(context);
        let uuid = Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6]);
        RFile::new(
            uuid,
            file_path
                .file_name()
                .unwrap_or(&OsStr::new("unamed"))
                .to_str()
                .unwrap_or("unamed")
                .to_string(),
            byte_end - byte_start,
            byte_start,
            byte_end,
        )
    }

    pub fn serialize(&self) -> String {
        let mut data = String::new();
        data.push_str(&self.uuid.to_string());
        data.push_str(",");
        data.push_str(&self.name);
        data.push_str(",");
        data.push_str(&self.byte_start.to_string());
        data.push_str(",");
        data.push_str(&self.byte_end.to_string());
        data
    }
}