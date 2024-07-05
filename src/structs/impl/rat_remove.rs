use crate::structs::rat_file::RatFile;
use memmap2::MmapMut;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[allow(dead_code)]
impl<'de, T> RatFile<T>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    pub(crate) fn remove(&mut self, id: Uuid) -> Result<(), std::io::Error> {
        let rat_file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .append(false)
            .open(self.file_path.clone())?;
        println!("rat_file_len: {:?}", rat_file.metadata()?.len());

        let list = &self.list_rat_file()?;

        for (i, file) in list.iter().enumerate() {
            if file.id != id {
                println!("|{}| file.id: {}, id: {}", i, file.id, id);
                continue;
            } else {
                println!("> |{}| file.id: {}, id: {}", i, file.id, id);
            }

            let start = file.start as usize;
            let end = file.end as usize;
            println!("start: {}, end: {}", start, end);
            let length_to_remove = end - start;

            /* MEMAPPING */
            let file_len: usize;
            {
                // Memory map the file
                let mut mmap = unsafe { MmapMut::map_mut(&rat_file)? };
                println!("mmap.len(): {}", mmap.len());

                // Shift data after the `end` position to the `start` position
                file_len = mmap.len();
                println!("removing {} bytes", end - start);
                mmap.copy_within(end..file_len, start);
                println!("mmap.len(): {}", mmap.len());

                // Sync the changes to the file
                mmap.flush()?;
            }
            rat_file.set_len((file_len - length_to_remove) as u64)?;
            println!("yay");
            println!("rat_file_len: {:?}", rat_file.metadata()?.len());
            // Remove the entry from the metadata
            {
                let file_info = 'x';
            }
            //TODO: define .files
            //self.files.remove(i);

            //TODO: remove headers of the file
            //TODO: reindex the other headers who were added after the removed file

            return Ok(());
        }

        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found",
        ));
    }
}
