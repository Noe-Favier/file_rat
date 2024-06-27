use crate::structs::{
    f_item::FileItem, rat_file::RatFile,
};

use base64::{alphabet, engine, read::DecoderReader};
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions}, io::{Read, Seek, SeekFrom}
};

#[allow(dead_code)]
impl<'de, T> RatFile<T>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    pub(crate) fn list_rat_file(&self) -> Result<Vec<FileItem<T>>, std::io::Error> {
        let mut rat_file: File = OpenOptions::new()
            .read(true)
            .write(false)
            .append(false)
            .open(self.file_path.clone())?;

        let mut items: Vec<FileItem<T>> = Vec::new();
        let mut header_index = self.get_item_header_index()?;
        let mut buffer = vec![0; Self::BUFFER_SIZE_HEADERS];

        let mut bytes_read: usize;

        rat_file.seek(SeekFrom::Start(header_index))?;
        println!("_- seeking through {:?}", header_index);
        loop {
            //TODO: increment a string because the buffer is overwritten each time making a partial b64 str which does not work
            bytes_read = rat_file.read(&mut buffer)?;
            println!("bytes read: {:?}", bytes_read);
            if bytes_read == 0 {
                break;
            }
            println!("buffer: {:?}", buffer.iter().map(|b| *b as char).collect::<String>());

            if let Some(pos) = buffer[..bytes_read].windows([Self::HEADER_ITEM_SEPARATOR].len()).position(|window| window == &[Self::HEADER_ITEM_SEPARATOR]) {
                println!("found separator at {:?}", pos);
                let item_data = &buffer[..pos];
                let engine = engine::GeneralPurpose::new(&alphabet::URL_SAFE, engine::general_purpose::PAD);
                let b64_decoder = DecoderReader::new(item_data, &engine);
                let mut decoded_string = String::new();
                let mut b64_reader = b64_decoder.take(pos as u64);
                b64_reader.read_to_string(&mut decoded_string)?;
                println!("decoded string: {:?}", decoded_string);
                if let Ok(file_item) = serde_json::from_str::<FileItem<T>>(&decoded_string) {
                    println!("file item: {:?}", file_item.id);
                    items.push(file_item);
                } else {
                    println!("failed to deserialize: {:?}", decoded_string);
                }
                rat_file.seek(SeekFrom::Current((pos + 1) as i64))?;
                buffer = vec![0; Self::BUFFER_SIZE_HEADERS];
            } else {
                header_index += bytes_read as u64;
                rat_file.seek(SeekFrom::Start(header_index))?;
            }
        }

        Ok(items)
    }
}
