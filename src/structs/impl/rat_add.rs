use crate::structs::{
    enums::compression_type::CompressionType, f_item::FileItem, rat_file::RatFile,
};

use base64::{alphabet, engine, write};
use bzip2::bufread::BzEncoder;
use memmap2::MmapMut;
use std::{
    borrow::{Borrow, BorrowMut}, fs::{File, OpenOptions}, io::{BufReader, Read, Seek, SeekFrom, Write}, path::PathBuf
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
        let mut mmap = unsafe { MmapMut::map_mut(&rat_file)? };
        let mut rat_file_len = rat_file.metadata()?.len();
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

        let general_header_index = self.get_general_header_index()? as usize;
        let mut write_pos = general_header_index;
        loop {
            let bytes_read = encoder.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            end += bytes_read;
            rat_file_len += bytes_read as u64;
            rat_file.set_len(rat_file_len)?; // extend file size
            println!("general hd {} & bytes read: {} & rat_file_len: {} => {:?}", general_header_index, bytes_read, rat_file_len, write_pos..write_pos + bytes_read);
            mmap[write_pos..write_pos + bytes_read].copy_from_slice(&buffer[..bytes_read]);
            write_pos += bytes_read;
        }

        // Flush changes to disk
        mmap.flush()?;

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
