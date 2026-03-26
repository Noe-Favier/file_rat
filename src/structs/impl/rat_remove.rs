use std::io::{Seek, Write};

use crate::structs::rat_file::RatFile;
use base64::{alphabet, engine, Engine as _};
use memmap2::MmapMut;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[allow(dead_code)]
impl<'de, T> RatFile<T>
where
    T: Serialize + for<'a> Deserialize<'a> + Clone,
{
    pub(crate) fn remove(&mut self, id: Uuid) -> Result<(), std::io::Error> {
        let mut rat_file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .append(false)
            .open(self.file_path.clone())?;

        let mut list = self.list_rat_file()?;
        let Some(target_file_index) = list.iter().position(|file| file.id == id) else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found",
            ));
        };

        let target = list[target_file_index].clone();
        let start = target.start as usize;
        let end = target.end as usize;
        if end < start {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid file bounds",
            ));
        }
        let length_to_remove = end - start;

        let old_general_header_index = self.get_general_header_section_index()?;

        /* MEMAPPING */
        let file_len: usize;
        {
            // Memory map the file
            let mut mmap = unsafe { MmapMut::map_mut(&rat_file)? };

            // Shift data after the `end` position to the `start` position
            file_len = mmap.len();
            if end > file_len {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid file bounds",
                ));
            }
            mmap.copy_within(end..file_len, start);

            // Sync the changes to the file
            mmap.flush()?;
        }
        rat_file.set_len((file_len - length_to_remove) as u64)?;

        // Remove the entry from metadata and reindex all following file entries.
        list.remove(target_file_index);
        for file in list.iter_mut() {
            if file.start >= target.end {
                file.start -= length_to_remove as u64;
                file.end -= length_to_remove as u64;
            }
        }

        // Rebuild the header section: keep the current general header bytes and rewrite item headers.
        let new_general_header_index =
            old_general_header_index.saturating_sub(length_to_remove as u64);
        let new_item_header_index = self.get_item_header_section_index()?;

        let general_header_len = new_item_header_index.saturating_sub(new_general_header_index);
        let mut general_header = vec![0u8; general_header_len as usize];
        rat_file.seek(std::io::SeekFrom::Start(new_general_header_index))?;
        std::io::Read::read_exact(&mut rat_file, &mut general_header)?;

        rat_file.set_len(new_general_header_index)?;
        rat_file.seek(std::io::SeekFrom::End(0))?;
        rat_file.write_all(&general_header)?;

        let b64_engine =
            engine::GeneralPurpose::new(&alphabet::URL_SAFE, engine::general_purpose::PAD);
        for file_item in list.iter() {
            let header = serde_json::to_vec(file_item)?;
            let encoded_header = b64_engine.encode(header);
            rat_file.write_all(encoded_header.as_bytes())?;
            rat_file.write_all(&[Self::HEADER_ITEM_SEPARATOR])?;
        }
        rat_file.flush()?;

        self.files = list;
        Ok(())
    }
}
