use crate::structs::rat_file::RatFile;
use crate::structs::enums::compression_type::CompressionType;
use bzip2::read::BzDecoder;
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{Error, ErrorKind, Read, Seek, SeekFrom, Write},
    path::PathBuf,
};
use uuid::Uuid;

#[allow(dead_code)]
impl<'de, T> RatFile<T>
where
    T: Serialize + for<'a> Deserialize<'a> + Clone,
{
    pub(crate) fn extract(
        &mut self,
        id: Uuid,
        file_dest: PathBuf,
        should_remove: bool,
    ) -> Result<PathBuf, Error> {
        let list = self.list_rat_file()?;
        let Some(target) = list.iter().find(|file| file.id == id) else {
            return Err(Error::new(ErrorKind::NotFound, "File not found"));
        };

        if target.end < target.start {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid file bounds"));
        }

        let output_path = if file_dest.is_dir() {
            file_dest.join(&target.name)
        } else {
            file_dest
        };

        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut rat_file = OpenOptions::new()
            .read(true)
            .write(false)
            .append(false)
            .open(self.file_path.clone())?;

        rat_file.seek(SeekFrom::Start(target.start))?;
        let mut encoded_data = vec![0u8; (target.end - target.start) as usize];
        rat_file.read_exact(&mut encoded_data)?;

        let item_compression = target
            .compression_type
            .and_then(CompressionType::from_u8)
            .unwrap_or(self.compression_type.clone());

        let mut bz_decoder = BzDecoder::new(&encoded_data[..]);
        let mut output_file = File::create(&output_path)?;
        match item_compression {
            CompressionType::Fast | CompressionType::Best | CompressionType::Default => {
                std::io::copy(&mut bz_decoder, &mut output_file)?;
            }
        }
        output_file.flush()?;

        if should_remove {
            self.remove(id)?;
        }

        Ok(output_path)
    }
}
