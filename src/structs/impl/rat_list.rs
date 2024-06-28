use crate::structs::{f_item::FileItem, rat_file::RatFile};

use base64::{alphabet, engine, read::DecoderReader};
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Cursor, Read, Seek, SeekFrom},
};

#[allow(dead_code)]
impl<'de, T> RatFile<T>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    pub(crate) fn list_rat_file(&self) -> Result<Vec<FileItem<T>>, std::io::Error> {
        let engine = engine::GeneralPurpose::new(&alphabet::URL_SAFE, engine::general_purpose::PAD);
        let header_index = self.get_item_header_index()?;
        let rat_file: File = OpenOptions::new()
            .read(true)
            .write(false)
            .append(false)
            .open(self.file_path.clone())?;

        let mut items: Vec<FileItem<T>> = Vec::new();
        let mut rat_bufread = BufReader::new(rat_file);
        let mut bytes_read: usize;
        let mut buffer = Vec::<u8>::new();
        let mut b64_decoder: DecoderReader<engine::GeneralPurpose, Cursor<&Vec<u8>>>;
        let mut decoded_string = String::new();

        rat_bufread.seek(SeekFrom::Start(header_index))?;
        loop {
            // encoded reading
            bytes_read = rat_bufread.read_until(Self::HEADER_ITEM_SEPARATOR,  &mut buffer)?;
            buffer.pop();
            if bytes_read == 0 {
                break;
            }
            // \\

            // decoding
            b64_decoder = DecoderReader::new(Cursor::new(&buffer), &engine);
            b64_decoder.read_to_string(&mut decoded_string)?;
            buffer.clear();
            // \\

            // deserializing
            let item = serde_json::from_str::<FileItem<T>>(&decoded_string)?;
            decoded_string.clear();
            items.push(item);
            // \\
        }
        Ok(items)
    }
}
