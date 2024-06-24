use crate::structs::{
    enums::compression_type::CompressionType, f_item::FileItem, rat_file::RatFile
};
use binrw::io::BufReader;
use bzip2::bufread::BzEncoder;
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
};

#[allow(dead_code)]
impl<T> RatFile<T> {
    pub(crate) fn insert_to_rat_file(
        &mut self,
        filep: PathBuf,
        metadata: T,
    ) -> Result<FileItem<T>, std::io::Error> {
        let buffer_size = Self::BUFFER_SIZE;
        // rat file descriptor
        let mut rat_file: File = File::open(self.file_path.clone())?;
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
        let mut encoder = BzEncoder::new(br, match self.compression_type {
            CompressionType::Fast => bzip2::Compression::fast(),
            CompressionType::Best => bzip2::Compression::best(),
            CompressionType::None => bzip2::Compression::none(),
        });
        // \\

        loop {
            let bytes_read = encoder.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            end += bytes_read;
            rat_file.write_all(&buffer[..bytes_read])?;
        }

        // ----- ----- ----- Header ----- ----- ----- //

        let header_start = Self::get_header_start(&mut rat_file)?;
        rat_file.seek(SeekFrom::Start(header_start))?;

        return Ok(FileItem::new(name, metadata, file_size, start, end as u64));
    }

}
