use crate::structs::{
    enums::compression_type::CompressionType, f_item::FileItem, rat_file::RatFile,
};

use base64::{alphabet, engine, write};
use bzip2::bufread::BzEncoder;
use std::{
    fs::{File, OpenOptions}, io::{BufReader, Read, Seek, SeekFrom, Write}, path::PathBuf
};

#[allow(dead_code)]
impl<T> RatFile<T> {
    pub(crate) fn insert_to_rat_file(
        &mut self,
        filep: PathBuf,
        metadata: T,
    ) -> Result<FileItem<T>, std::io::Error> {
        let buffer_size = Self::BUFFER_SIZE;
        // rat file descriptor (opened with write permissions)
        let mut rat_file: File = OpenOptions::new()
            .write(true)
            .append(true)
            .read(true)
            .open(self.file_path.clone())?;
        // \\

        // file descriptor
        let file: File = File::open(filep.clone())?;
        // \\

        // FileItem attributes
        let name = filep
            .file_name()
            .unwrap()
            .to_str()
            .unwrap_or("untilted")
            .to_string();
        let file_size = file.metadata()?.len();
        let mut end = 0; // will be incremented as we read the file
        let start = rat_file.seek(SeekFrom::End(0))?;
        // \\

        // ----- ----- -----  DATA  ----- ----- ----- //
        // Encoding utils
        let mut buffer = vec![0; buffer_size];
        let br: BufReader<File> = BufReader::new(file);
        let mut encoder = BzEncoder::new(
            br,
            match self.compression_type {
                CompressionType::Fast => bzip2::Compression::fast(),
                CompressionType::Best => bzip2::Compression::best(),
                CompressionType::Default => bzip2::Compression::default(),
            },
        );
        // \\

        
        // Move the general header to a tmp file
        rat_file.seek(SeekFrom::End(self.get_general_header_index()? as i64))?; //TODO: check if this is correct
        let header_old_pos = rat_file.seek(SeekFrom::Current(0))?;
        println!("header_old_pos: {} // {}", header_old_pos, self.get_general_header_index()?);
        let mut tmpfile: File = tempfile::tempfile()?;
        loop {
            let bytes_read = rat_file.read(&mut buffer)?;
            println!("(1) bytes_read: {}", bytes_read);
            if bytes_read == 0 {
                break;
            }
            tmpfile.write(&buffer[..bytes_read])?;
        }

        // Append data to the rat file
        rat_file.seek(SeekFrom::Start(header_old_pos))?;
        println!("writing data to rat file at position {}", rat_file.seek(SeekFrom::Current(0))?);
        loop {
            let bytes_read = encoder.read(&mut buffer)?;
            println!("(2) bytes_read: {}", bytes_read);
            if bytes_read == 0 {
                break;
            }
            end += bytes_read;
            rat_file.write(&buffer[..bytes_read])?;
        }

        // Append all headers back to the rat file
        tmpfile.seek(SeekFrom::Start(0))?;
        rat_file.seek(SeekFrom::End(0))?;
        loop {
            let bytes_read = tmpfile.read(&mut buffer)?;
            println!("(3) bytes_read: {}", bytes_read);
            if bytes_read == 0 {
                break;
            }
            rat_file.write(&buffer[..bytes_read])?;
        }


        // ----- ----- ----- Header ----- ----- ----- //
        let header_start = self.get_item_header_index()?;
        rat_file.seek(SeekFrom::Start(header_start))?;

        // header
        let header = b"{header}";
        //encode the header in base64
        let engine = engine::GeneralPurpose::new(&alphabet::URL_SAFE, engine::general_purpose::PAD);
        let mut b64_encoder = write::EncoderWriter::new(rat_file, &engine);
        b64_encoder.write_all(header)?;
        // \\

        return Ok(FileItem::new(name, metadata, file_size, start, end as u64));
    }
}
